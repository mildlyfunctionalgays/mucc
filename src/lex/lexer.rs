pub use super::constants::*;
use crate::lex::errors::LexError;
use crate::lex::errors::LexErrorType;
use crate::lex::errors::LexResult;
use crate::lex::errors::LexSuccess;
use crate::lex::errors::Location;
use crate::lex::Lexer;
use std::char;
use std::iter::Iterator;
use std::str::FromStr;

const INVALID_IDENTIFIER_CHARS: &str = " !\"#%&'()*+,-./;;<=>?@[\\]^`{|}~";

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
            line: 1,
            column: 0,
            start_line: 1,
            start_column: 0,
            last_column: 0,
        }
    }

    fn set_start_pos(&mut self) -> Option<()> {
        let ch = self.next_after_whitespace()?;
        self.start_line = self.line;
        self.start_column = self.column;
        self.nextnt(ch);
        Some(())
    }

    pub(super) fn next_char(&mut self) -> Option<char> {
        let ch = self.lookahead.pop().or_else(|| self.source.next())?;
        match ch {
            '\n' => {
                self.last_column = self.column;
                self.line += 1;
                self.column = 0;
            }
            _ => self.column += 1,
        }
        Some(ch)
    }

    fn next_chars(&mut self, n: usize) -> Option<String> {
        let next = (0..n).filter_map(|_| self.next_char()).collect::<String>();
        if next.len() < n {
            self.nextnt_string(&next);
            None
        } else {
            Some(next)
        }
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
        let ch = self.skip_chars(" \n\t\r")?;
        self.nextnt(ch);

        let next = self.next_chars(2);

        if next.as_ref().map(|s| s.as_str()) == Some("//") {
            while self.next_char()? != '\n' {}
            self.next_after_whitespace()
        } else {
            if let Some(next) = next {
                self.nextnt_string(&next);
            }
            self.next_char()
        }
    }
    pub(super) fn nextnt(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.line -= 1;
                self.column = self.last_column;
            }
            _ => self.column -= 1,
        }
        self.lookahead.push(ch);
    }

    pub(super) fn nextnt_string(&mut self, s: &str) {
        s.chars().rev().for_each(|c| self.nextnt(c));
    }

    fn next_regular_token(&mut self) -> Option<LexResult> {
        let mut token: String = self.next_after_whitespace()?.to_string();

        loop {
            let partial_matches: Vec<&(&str, LexItem)> = LITERAL_TOKENS
                .iter()
                .filter(|(key, _)| key.trim_end_matches('\x00').starts_with(&token))
                .collect();

            let returning_match = partial_matches.len() < 2
                && if let Some((match_, _)) = partial_matches.first() {
                    token.starts_with(match_)
                } else {
                    true
                };

            if !returning_match {
                if let Some(ch) = self.next_char() {
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
            return if let Some((key, value)) = largest_match {
                self.nextnt_string(&token[key.trim_end_matches('\x00').len()..]);
                Some(Ok(self.ok_token(value.clone())))
            } else {
                self.nextnt_string(&token);
                None
            };
        }
    }

    fn parse_escape_sequence(&mut self) -> Result<u32, LexError> {
        Ok(match self.next_char().unwrap_or_else(|| unimplemented!()) {
            'a' => '\x07',
            'b' => '\x08',
            'f' => '\r',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'v' => '\x0B',
            '\\' => '\\',
            '\'' => '\'',
            '"' => '"',
            '?' => '?',
            ch @ '0'..='9' => {
                let mut number_str = ch.to_string();
                number_str.push_str(&self.next_chars(3).unwrap_or_else(|| unimplemented!()));
                return u32::from_str(&number_str)
                    .map_err(|_| self.error_token(LexErrorType::InvalidEscape(number_str)));
            }
            'x' => {
                return u32::from_str_radix(
                    &self.next_chars(2).unwrap_or_else(|| unimplemented!()),
                    16,
                )
                .map_err(|_| unimplemented!());
            }
            'e' => '\x1B',
            'U' => unimplemented!(),
            'u' => {
                let s = self.next_chars(4).unwrap_or_else(|| unimplemented!());
                return u32::from_str(&s)
                    .map_err(|_| self.error_token(LexErrorType::InvalidEscape(format!("u{}", s))));
            }
            invalid => {
                return Err(self.error_token(LexErrorType::InvalidEscape(invalid.to_string())));
            }
        } as u32)
    }

    fn parse_char_literal(&mut self) -> LexResult {
        let r = match self
            .next_char()
            .ok_or_else(|| self.error_token(LexErrorType::Unfinished("'".to_string())))?
        {
            '\'' => return Err(self.error_token(LexErrorType::InvalidLiteral("''".to_string()))),
            '\\' => self.parse_escape_sequence()?,
            ch => ch as u32,
        };
        let next = self.next_char().ok_or_else(|| {
            self.error_token(LexErrorType::Unfinished(format!(
                "'{}",
                char::from_u32(r).unwrap()
            )))
        })?;
        if next == '\'' {
            Ok(self.ok_token(LexItem::NumericLiteral(NumberType::UnsignedInt(r))))
        } else {
            Err(self.error_token(LexErrorType::InvalidLiteral("".to_string())))
        }
    }

    pub(super) fn ok_token(&self, token: LexItem) -> LexSuccess {
        LexSuccess {
            item: token,
            line: self.start_line,
            column: self.start_column,
        }
    }

    pub(super) fn error_token(&self, token: LexErrorType) -> LexError {
        LexError {
            error_type: token,
            location: Location {
                line: self.start_line,
                column: self.start_column,
            },
        }
    }

    pub(super) fn parse_type_specifier(&mut self, num: u128) -> LexResult {
        let mut signed = true;
        let mut size = 32usize;
        while let Some(ch) = self.next_char() {
            match ch.to_ascii_lowercase() {
                'u' => signed = false,
                'l' => size <<= 1,
                'a'...'z' => {
                    return Err(self.error_token(LexErrorType::InvalidLiteral(format!("'{}'", ch))));
                }
                _ => {
                    self.nextnt(ch);
                    break;
                }
            }
        }
        let nt = match (size, signed) {
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
            _ => return Err(self.error_token(LexErrorType::InvalidSize(size))),
        };

        Ok(self.ok_token(LexItem::NumericLiteral(nt)))
    }
}

impl<It> Iterator for Lexer<It>
where
    It: Iterator<Item = char>,
{
    type Item = LexResult;

    fn next(&mut self) -> Option<LexResult> {
        self.set_start_pos()?;

        if let Some(token) = self.next_regular_token() {
            Some(token)
        } else {
            let ch = self.next_after_whitespace()?;
            Some((|| match ch {
                '"' => {
                    let mut s: Vec<u8> = Vec::new();
                    loop {
                        let ch = match self.next_char() {
                            Some(ch) => ch,
                            None => {
                                return Err(self.error_token(LexErrorType::UnclosedStringLiteral(
                                    String::from_utf8_lossy(&s).to_string(),
                                )));
                            }
                        };
                        let mut buffer = [0u8; 4];
                        match ch {
                            '"' => break,
                            '\\' => {
                                let character = self.parse_escape_sequence()?;
                                if let Some(character) = char::from_u32(character) {
                                    s.extend_from_slice(
                                        character.encode_utf8(&mut buffer).as_bytes(),
                                    );
                                } else {
                                    s.push(character as u8);
                                }
                            }
                            '\n' => {
                                return Err(self.error_token(LexErrorType::UnclosedStringLiteral(
                                    String::from_utf8_lossy(&s).to_string(),
                                )));
                            }
                            _ => s.extend_from_slice(ch.encode_utf8(&mut buffer).as_bytes()),
                        }
                    }
                    Ok(self.ok_token(LexItem::StringLiteral(s)))
                }
                '0' => self.parse_numeric_zero_literal(),
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

                    self.parse_type_specifier(
                        u128::from_str(&num)
                            .ok()
                            .unwrap_or_else(|| unimplemented!()),
                    )
                }
                '\'' => self.parse_char_literal(),
                _ => {
                    if INVALID_IDENTIFIER_CHARS.chars().any(|c| c == ch) {
                        Err(self.error_token(LexErrorType::InvalidCharacter(ch)))
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

                        Ok(self.ok_token(LexItem::Identifier(ident)))
                    }
                }
            })())
        }
    }
}
