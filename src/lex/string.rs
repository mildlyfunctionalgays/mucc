use crate::lex::errors::LexError;
use crate::lex::errors::LexErrorType;
use crate::lex::errors::LexResult;
use crate::lex::types::LexItem;
use crate::lex::types::NumberType;
use crate::lex::Lexer;
use std::char;
use std::str::FromStr;

impl<It: Iterator<Item = char>> Lexer<It> {
    pub(super) fn parse_escape_sequence(&mut self) -> Result<u32, LexError> {
        Ok(match self
            .next_char()
            .ok_or_else(|| self.error_token(LexErrorType::UnfinishedEscape))?
            .ch
        {
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
                number_str.push_str(
                    &self
                        .next_chars(3)
                        .ok_or_else(|| self.error_token(LexErrorType::UnfinishedEscape))?
                        .to_string(),
                );
                return u32::from_str(&number_str)
                    .map_err(|_| self.error_token(LexErrorType::InvalidEscape(number_str)));
            }
            'x' => {
                let chars = self
                    .next_chars(2)
                    .ok_or_else(|| self.error_token(LexErrorType::UnfinishedEscape))?;
                return u32::from_str_radix(&chars.to_string(), 16).map_err(|_| {
                    self.error_token(LexErrorType::InvalidEscape(
                        r"\x".to_string() + &chars.to_string(),
                    ))
                });
            }
            'e' => '\x1B',
            'U' => {
                let s = self
                    .next_chars(8)
                    .ok_or_else(|| self.error_token(LexErrorType::UnfinishedEscape))?;
                let string = s.to_string();
                return u32::from_str(&string).map_err(|_| {
                    self.error_token(LexErrorType::InvalidEscape(format!("U{}", string)))
                });
            }
            'u' => {
                let s: String = self
                    .next_chars(4)
                    .ok_or_else(|| self.error_token(LexErrorType::UnfinishedEscape))?
                    .to_string();
                return u32::from_str(&s)
                    .map_err(|_| self.error_token(LexErrorType::InvalidEscape(format!("u{}", s))));
            }
            invalid => {
                return Err(self.error_token(LexErrorType::InvalidEscape(invalid.to_string())));
            }
        } as u32)
    }

    pub(super) fn parse_char_literal(&mut self) -> LexResult {
        let r = match self
            .next_char()
            .ok_or_else(|| self.error_token(LexErrorType::Unfinished("'".to_string())))?
            .ch
        {
            '\'' => return Err(self.error_token(LexErrorType::InvalidLiteral("''".to_string()))),
            '\\' => self.parse_escape_sequence()?,
            ch => ch as u32,
        };
        let next = self
            .next_char()
            .ok_or_else(|| {
                self.error_token(LexErrorType::Unfinished(format!(
                    "'{}",
                    char::from_u32(r).unwrap()
                )))
            })?
            .ch;
        if next == '\'' {
            Ok(self.ok_token(LexItem::NumericLiteral(NumberType::UnsignedInt(r))))
        } else {
            Err(self.error_token(LexErrorType::InvalidLiteral("".to_string())))
        }
    }

    pub(super) fn parse_string_literal(&mut self) -> LexResult {
        let mut s: Vec<u8> = Vec::new();
        loop {
            let ch = self
                .next_char()
                .ok_or_else(|| {
                    self.error_token(LexErrorType::UnclosedStringLiteral(
                        String::from_utf8_lossy(&s).to_string(),
                    ))
                })?
                .ch;
            let mut buffer = [0u8; 4];
            match ch {
                '"' => break,
                '\\' => {
                    let character = self.parse_escape_sequence()?;
                    if let Some(character) = char::from_u32(character) {
                        s.extend_from_slice(character.encode_utf8(&mut buffer).as_bytes());
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
}
