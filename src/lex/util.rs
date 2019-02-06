use crate::lex::constants::LexItem;
use crate::lex::errors::LexError;
use crate::lex::errors::LexErrorType;
use crate::lex::errors::LexSuccess;
use crate::lex::errors::Location;
use crate::lex::Lexer;

impl<It: Iterator<Item = char>> Lexer<It> {
    pub(super) fn set_start_pos(&mut self) -> Option<()> {
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

    pub(super) fn next_chars(&mut self, n: usize) -> Option<String> {
        let next = (0..n).filter_map(|_| self.next_char()).collect::<String>();
        if next.len() < n {
            self.nextnt_string(&next);
            None
        } else {
            Some(next)
        }
    }

    pub(super) fn skip_chars(&mut self, chars: &str) -> Option<char> {
        loop {
            let ch = self.next_char()?;
            if !chars.chars().any(|c| c == ch) {
                break Some(ch);
            }
        }
    }
    pub(super) fn next_after_whitespace(&mut self) -> Option<char> {
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
}
