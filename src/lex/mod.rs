pub mod constants;
pub mod errors;
#[cfg(test)]
mod tests;

pub struct Lexer<It: Iterator<Item = char>> {
    source: It,
    lookahead: Vec<char>,
    line: usize,
    column: usize,
    start_line: usize,
    start_column: usize,
    last_column: usize,
}

pub mod lexer;
mod numeric;
