use crate::lex::constants::LexItem;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct LexError {
    pub error_type: LexErrorType,
    pub location: Location,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexErrorType {
    Unfinished(String),
    InvalidEscape(String),
    InvalidLiteral(String),
    InvalidSize(usize),
    InvalidCharacter(char),
    Other(String),
}

impl Display for LexErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            LexErrorType::Unfinished(comment) => write!(f, "Unfinished statement: {}", comment),
            LexErrorType::InvalidEscape(comment) => {
                write!(f, "Invalid escape sequence: {}", comment)
            }
            LexErrorType::InvalidLiteral(comment) => write!(f, "Invalid literal: {}", comment),
            LexErrorType::InvalidSize(comment) => write!(f, "Invalid size: {} bits long", comment),
            LexErrorType::Other(comment) => write!(f, "Other lex error: {}", comment),
            _ => unimplemented!(),
        }
    }
}

pub type LexResult = Result<LexSuccess, LexError>;

#[derive(Clone, Debug, PartialEq)]
pub struct LexSuccess {
    pub item: LexItem,
    pub line: usize,
    pub column: usize,
}
