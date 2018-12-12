pub use super::constants::*;
use std::iter::Iterator;
use std::str::FromStr;

pub struct Lexer<It: Iterator<Item = char>> {
    source: It,
    lookahead: Vec<char>,
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
            let exact_match = LITERAL_TOKENS
                .iter()
                .find(|(key, _)| token == *key)
                .map(|(_, value)| value);
            let partial_matches: Vec<&LexItem> = LITERAL_TOKENS
                .iter()
                .filter_map(|(key, val)| {
                    if key.starts_with(&token) {
                        Some(val)
                    } else {
                        None
                    }
                })
                .collect();

            let mut too_much = match partial_matches.len() {
                1 => {
                    if let Some(match_) = exact_match {
                        break match_.clone();
                    } else {
                        false
                    }
                }
                0 => true,
                _ => false,
            };

            // This means that we shouldn't be treating it as a normal token
            // It only happens if there are no partial matches and we have only one character
            // This will need changing if there are to be normal tokens which include ASCII
            // or Unicode characters valid in identifiers, keywords, or literals
            if too_much && token.len() == 1 {
                self.nextnt_string(&token);
                return None;
            }
            if !too_much {
                if let Some(char) = self.next_char() {
                    token.push(char);
                } else {
                    too_much = true;
                }
            }

            if too_much {
                let largest_match = LITERAL_TOKENS
                    .iter()
                    .filter(|(key, _)| token.starts_with(key))
                    .max_by_key(|(key, _)| key.len());
                let (key, value) = largest_match?;
                self.nextnt_string(&token[key.len()..]);
                break value.clone();
            }
        })
    }

    fn parse_char_literal(&mut self) -> Option<LexItem> {
        let r = match self.next_char()? {
            '\'' => None,
            '\\' => match self.next_char()? {
                'n' => Some(b'\n'),
                't' => Some(b'\t'),
                'r' => Some(b'\r'),
                '\\' => Some(b'\\'),
                '\'' => Some(b'\''),
                'x' => u8::from_str_radix(&self.next_chars(2)?, 16).ok(),
                _ => unimplemented!(),
            },
            ch => {
                if (ch as u32) <= 0x7f {
                    // This is one byte in utf8
                    Some(ch as u8)
                } else {
                    None
                }
            }
        };
        if self.next_char()? == '\'' {
            Some(LexItem::NumericLiteral(NumberType::UnsignedChar(r?)))
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
            println!("ch is {}, lookahead is {:?}", ch, self.lookahead);
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
                        '0'...'9' => unimplemented!(),
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
                _ => unimplemented!(),
            }
        })
    }
}
