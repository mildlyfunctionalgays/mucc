use crate::lex::constants::LexItem;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::result::Result::Ok;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexError {
    Unfinished(String),
    InvalidEscape(String),
    InvalidLiteral(String),
    InvalidSize(usize),
    Other(String),
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            LexError::Unfinished(comment) => write!(f, "Unfinished statement: {}", comment),
            LexError::InvalidEscape(comment) => write!(f, "Invalid escape sequence: {}", comment),
            LexError::InvalidLiteral(comment) => write!(f, "Invalid literal: {}", comment),
            LexError::InvalidSize(comment) => write!(f, "Invalid size: {} bits long", comment),
            LexError::Other(comment) => write!(f, "Other lex error: {}", comment),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LexResult {
    pub item: Result<LexItem, LexError>,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LexSuccess {
    pub item: LexItem,
    pub line: usize,
    pub column: usize,
}

impl Display for LexResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.item {
            Ok(item) => {
                write!(
                    f,
                    "Token on line {} column {}: {:?}",
                    self.line, self.column, item
                )?;
            }
            Err(err) => {
                write!(
                    f,
                    "Lex error on line {} column {}: {}",
                    self.line, self.column, err
                )?;
            }
        }
        Ok(())
    }
}

impl LexResult {
    pub fn ok(self) -> Option<LexSuccess> {
        if let Ok(item) = self.item {
            Some(LexSuccess {
                item,
                line: self.line,
                column: self.column,
            })
        } else {
            None
        }
    }

    pub fn is_err(&self) -> bool {
        self.item.is_err()
    }
}

impl From<LexSuccess> for LexResult {
    fn from(item: LexSuccess) -> Self {
        LexResult {
            item: Ok(item.item),
            line: item.line,
            column: item.column,
        }
    }
}
