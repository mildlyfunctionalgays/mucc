pub(crate) mod constants;
pub mod errors;
#[cfg(test)]
mod tests;
pub mod types;

pub struct Lexer<It: Iterator<Item = char>> {
    source: It,
    lookahead: Vec<char>,
    line: usize,
    column: usize,
    start_line: usize,
    start_column: usize,
    last_column: usize,
}

// Implementations of lexer
mod identifier;
mod iterator_util;
pub mod lexer;
mod numeric;
mod string;
