pub use super::constants::*;
use std::iter::Iterator;
use std::str::FromStr;

const INVALID_IDENTIFIER_CHARS: &str = " !\"#%&'()*+,-./;;<=>?@[\\]^`{|}~";

pub struct Lexer<It: Iterator<Item = char>> {
    source: It,
    lookahead: Vec<char>,
}

fn is_identifier_char(ch: char) -> bool {
    !INVALID_IDENTIFIER_CHARS.chars().any(|c| c == ch)
}

impl<It> Lexer<It>
where
    It: Iterator<Item = char>,
{
    pub fn new(src: It) -> Lexer<It> {
        Lexer {
            source: src,
            lookahead: Vec::new(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.lookahead.pop().or_else(|| self.source.next())
    }

    fn next_chars(&mut self, n: usize) -> Option<String> {
        (0..n).map(|_| self.next_char()).collect::<Option<String>>()
    }

    fn skip_chars(&mut self, chars: &str) -> Option<char> {
        loop {
            let ch = self.next_char()?;
            if !chars.chars().any(|c| c == ch) {
                break Some(ch);
            }
        }
    }
    fn next_after_whitespace(&mut self) -> Option<char> {
        self.skip_chars(" \n\t\r")
    }
    fn nextnt(&mut self, ch: char) {
        self.lookahead.push(ch);
    }

    fn nextnt_string(&mut self, s: &str) {
        self.lookahead.extend(s.chars().rev());
    }

    fn next_regular_token(&mut self) -> Option<LexItem> {
        let mut token: String = self.next_after_whitespace()?.to_string();

        Some(loop {
            let partial_matches: Vec<&(&str, LexItem)> = LITERAL_TOKENS
                .iter()
                .filter(|(key, val)| key.trim_end_matches('\x00').starts_with(&token))
                .collect();

            println!("{:?}: {:?}", token, partial_matches);

            let returning_match = partial_matches.len() < 2
                && if let Some((match_, _)) = partial_matches.first() {
                    token.starts_with(match_)
                } else {
                    true
                };

            if !returning_match {
                if let Some(ch) = self.next_char() {
                    println!("added {:?}", ch);
                    token.push(ch);
                    continue;
                }
            }

            let largest_match = LITERAL_TOKENS
                .iter()
                .filter(|(key, _)| {
                    token.starts_with(key.trim_end_matches('\x00'))
                        && if key.ends_with('\x00') {
                            token
                                .trim_start_matches(key.trim_end_matches('\x00'))
                                .chars()
                                .next()
                                .map(|ch| !is_identifier_char(ch))
                                .unwrap_or(true)
                        } else {
                            true
                        }
                })
                .max_by_key(|(key, _)| key.len());
            if let Some((key, value)) = largest_match {
                self.nextnt_string(&token[key.trim_end_matches('\x00').len()..]);
                break value.clone();
            } else {
                self.nextnt_string(&token);
                return None;
            }
        })
    }

    fn parse_char_literal(&mut self) -> Option<LexItem> {
        let r: Option<u32> = match self.next_char()? {
            '\'' => None,
            '\\' => match self.next_char()? {
                'n' => Some(b'\n' as u32),
                't' => Some(b'\t' as u32),
                'r' => Some(b'\r' as u32),
                '\\' => Some(b'\\' as u32),
                '\'' => Some(b'\'' as u32),
                'x' => u32::from_str_radix(&self.next_chars(2)?, 16).ok(),
                _ => unimplemented!(),
            },
            ch => Some(ch as u32),
        };
        if self.next_char()? == '\'' {
            Some(LexItem::NumericLiteral(NumberType::UnsignedInt(r?)))
        } else {
            None
        }
    }

    fn parse_type_specifier(&mut self, num: u128) -> Option<NumberType> {
        let mut signed = true;
        let mut size = 32usize;
        while let Some(ch) = self.next_char() {
            match ch.to_ascii_lowercase() {
                'u' => signed = false,
                'l' => size <<= 1,
                'a'...'z' => return None,
                _ => {
                    self.nextnt(ch);
                    break;
                }
            }
        }
        Some(match (size, signed) {
            (8, false) => NumberType::UnsignedChar(num as u8),
            (8, true) => NumberType::SignedChar(num as i8),
            (16, false) => NumberType::UnsignedShort(num as u16),
            (16, true) => NumberType::SignedShort(num as i16),
            (32, false) => NumberType::UnsignedInt(num as u32),
            (32, true) => NumberType::SignedInt(num as i32),
            (64, false) => NumberType::UnsignedLong(num as u64),
            (64, true) => NumberType::SignedLong(num as i64),
            (128, false) => NumberType::UnsignedLongLong(num as u128),
            (128, true) => NumberType::SignedLongLong(num as i128),
            _ => return None,
        })
    }
}

impl<It> Iterator for Lexer<It>
where
    It: Iterator<Item = char>,
{
    type Item = LexItem;

    fn next(&mut self) -> Option<LexItem> {
        self.next_regular_token().or_else(|| {
            let mut ch = self.next_after_whitespace()?;
            match ch {
                '"' => unimplemented!(),
                '0' => {
                    ch = self.next_char()?;
                    match ch {
                        'b' => {
                            let mut num = String::new();
                            num.push(self.next_char()?);
                            while let Some(ch) = self.next_char() {
                                let chl = ch.to_ascii_lowercase();
                                if '0' == ch || ch == '1' {
                                    num.push(chl);
                                } else {
                                    self.nextnt(ch);
                                    break;
                                }
                            }

                            Some(LexItem::NumericLiteral(
                                self.parse_type_specifier(u128::from_str_radix(&num, 2).ok()?)?,
                            ))
                        }
                        'o' => {
                            let mut num = String::new();
                            num.push(self.next_char()?);
                            while let Some(ch) = self.next_char() {
                                let chl = ch.to_ascii_lowercase();
                                if '0' <= ch && ch <= '7' {
                                    num.push(chl);
                                } else {
                                    self.nextnt(ch);
                                    break;
                                }
                            }

                            Some(LexItem::NumericLiteral(
                                self.parse_type_specifier(u128::from_str_radix(&num, 8).ok()?)?,
                            ))
                        }
                        'x' => {
                            let mut num = String::new();
                            num.push(self.next_char()?);
                            while let Some(ch) = self.next_char() {
                                let chl = ch.to_ascii_lowercase();
                                if '0' <= ch && ch <= '9' || 'a' <= chl && chl <= 'f' {
                                    num.push(chl);
                                } else {
                                    self.nextnt(ch);
                                    break;
                                }
                            }

                            Some(LexItem::NumericLiteral(self.parse_type_specifier(
                                u128::from_str_radix(&num, 16).ok()?,
                            )?))
                        }
                        '0'...'9' => {
                            let mut num = String::new();
                            num.push(ch);
                            while let Some(ch) = self.next_char() {
                                let chl = ch.to_ascii_lowercase();
                                if '0' <= ch && ch <= '7' {
                                    num.push(chl);
                                } else {
                                    self.nextnt(ch);
                                    break;
                                }
                            }

                            Some(LexItem::NumericLiteral(
                                self.parse_type_specifier(u128::from_str_radix(&num, 8).ok()?)?,
                            ))
                        }
                        'U' | 'L' | 'u' | 'l' => {
                            self.nextnt(ch);
                            Some(LexItem::NumericLiteral(self.parse_type_specifier(0)?))
                        }
                        _ => {
                            self.nextnt(ch);
                            Some(LexItem::NumericLiteral(NumberType::SignedInt(0)))
                        }
                    }
                }
                '1'...'9' => {
                    let mut num = String::new();
                    num.push(ch);
                    while let Some(ch) = self.next_char() {
                        if '0' <= ch && ch <= '9' {
                            num.push(ch);
                        } else {
                            self.nextnt(ch);
                            break;
                        }
                    }

                    Some(LexItem::NumericLiteral(
                        self.parse_type_specifier(u128::from_str(&num).ok()?)?,
                    ))
                }
                '\'' => self.parse_char_literal(),
                _ => {
                    if INVALID_IDENTIFIER_CHARS.chars().any(|c| c == ch) {
                        None
                    } else {
                        let mut ident = String::new();
                        ident.push(ch);
                        while let Some(ch) = self.next_char() {
                            if !INVALID_IDENTIFIER_CHARS.chars().any(|c| c == ch) {
                                ident.push(ch);
                            } else {
                                self.nextnt(ch);
                                break;
                            }
                        }

                        Some(LexItem::Identifier(ident))
                    }
                }
            }
        })
    }
}
