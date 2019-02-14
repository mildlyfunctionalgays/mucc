use crate::lex::errors::Location;
use crate::lex::iterator_util::SourceChar;
use std::iter::Enumerate;

pub(crate) mod constants;
pub mod errors;
#[cfg(test)]
mod tests;
pub mod types;

pub struct Lexer<It: Iterator<Item = char>> {
    source: Enumerate<It>,
    lookahead: Vec<SourceChar>,
    character: Location,
    start_char: Location,
}

// Implementations of lexer
mod identifier;
mod iterator_util;
pub mod lexer;
mod numeric;
mod string;
