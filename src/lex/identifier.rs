use crate::lex::constants::LexItem;
use crate::lex::constants::INVALID_IDENTIFIER_CHARS;
use crate::lex::errors::LexErrorType;
use crate::lex::errors::LexResult;
use crate::lex::Lexer;

impl<It: Iterator<Item = char>> Lexer<It> {
    pub(super) fn parse_identifier(&mut self, ch: char) -> LexResult {
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
}
