use crate::lex::constants::LexItem;
use crate::lex::constants::NumberType;
use crate::lex::errors::LexErrorType;
use crate::lex::errors::LexResult;
use crate::lex::Lexer;

impl<It: Iterator<Item = char>> Lexer<It> {
    pub(super) fn parse_numeric_zero_literal(&mut self) -> LexResult {
        if let Some(ch) = self.next_char() {
            match ch {
                'b' => {
                    let mut num = String::new();
                    while let Some(ch) = self.next_char() {
                        let chl = ch.to_ascii_lowercase();
                        if '0' == ch || ch == '1' {
                            num.push(chl);
                        } else {
                            self.nextnt(ch);
                            break;
                        }
                    }

                    self.parse_num_radix(num, 2)
                }
                'o' => {
                    let mut num = String::new();
                    while let Some(ch) = self.next_char() {
                        let chl = ch.to_ascii_lowercase();
                        if '0' <= ch && ch <= '7' {
                            num.push(chl);
                        } else {
                            self.nextnt(ch);
                            break;
                        }
                    }
                    self.parse_num_radix(num, 8)
                }
                'x' => {
                    let mut num = String::new();
                    while let Some(ch) = self.next_char() {
                        let chl = ch.to_ascii_lowercase();
                        if '0' <= ch && ch <= '9' || 'a' <= chl && chl <= 'f' {
                            num.push(chl);
                        } else {
                            self.nextnt(ch);
                            break;
                        }
                    }
                    self.parse_num_radix(num, 16)
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

                    self.parse_num_radix(num, 8)
                }
                'U' | 'L' | 'u' | 'l' => {
                    self.nextnt(ch);
                    self.parse_type_specifier(0)
                }
                _ => {
                    self.nextnt(ch);
                    Ok(self.ok_token(LexItem::NumericLiteral(NumberType::SignedInt(0))))
                }
            }
        } else {
            Ok(self.ok_token(LexItem::NumericLiteral(NumberType::SignedInt(0))))
        }
    }

    fn parse_num_radix(&mut self, num: String, radix: u32) -> LexResult {
        if !num.is_empty() {
            if let Ok(n) = u128::from_str_radix(&num, radix) {
                self.parse_type_specifier(n)
            } else {
                Err(self.error_token(LexErrorType::LargeNumericLiteral))
            }
        } else {
            Err(self.error_token(LexErrorType::EmptyNumericLiteral))
        }
    }
}
