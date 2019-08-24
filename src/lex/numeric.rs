use crate::lex::errors::LexErrorType;
use crate::lex::errors::LexResult;
use crate::lex::types::LexItem;
use crate::lex::types::NumberType;
use crate::lex::Lexer;

impl<It: Iterator<Item = char>> Lexer<It> {
    pub(super) fn read_numeric_literal(&mut self, radix: u8) -> LexResult {
        let mut num = String::new();
        while let Some(ch) = self.next_char() {
            let chl = ch.ch.to_ascii_lowercase();
            match chl {
                '0'..='9' if (chl as u8) - b'0' < radix => num.push(chl),
                'a'..='z' if (chl as u8) - b'a' + 10 < radix => num.push(chl),
                _ => {
                    self.nextnt(ch);
                    break;
                }
            };
        }

        self.parse_num_radix(num, radix)
    }

    pub(super) fn parse_numeric_zero_literal(&mut self) -> LexResult {
        if let Some(ch) = self.next_char() {
            match ch.ch {
                'b' => self.read_numeric_literal(2),
                'o' => self.read_numeric_literal(8),
                'x' => self.read_numeric_literal(16),
                '0'..='9' => {
                    self.nextnt(ch);
                    self.read_numeric_literal(8)
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

    fn parse_num_radix(&mut self, num: String, radix: u8) -> LexResult {
        if !num.is_empty() {
            if let Ok(n) = u128::from_str_radix(&num, u32::from(radix)) {
                self.parse_type_specifier(n)
            } else {
                Err(self.error_token(LexErrorType::LargeNumericLiteral))
            }
        } else {
            Err(self.error_token(LexErrorType::EmptyNumericLiteral))
        }
    }

    fn parse_type_specifier(&mut self, num: u128) -> LexResult {
        let mut signed = true;
        let mut size = 32usize;
        while let Some(ch) = self.next_char() {
            match ch.ch.to_ascii_lowercase() {
                'u' => signed = false,
                'l' => size <<= 1,
                'a'..='z' => {
                    return Err(
                        self.error_token(LexErrorType::InvalidLiteral(format!("'{}'", ch.ch)))
                    );
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
