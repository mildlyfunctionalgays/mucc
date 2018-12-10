use std::iter::Iterator;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum LexItem {
    // Literals
    StringLiteral(String),
    NumericLiteral(String), // only return a string here so we can figure out the type later
    FloatLiteral(String),
    HexLiteral(String),

    // Operations
    Plus,   // Not necessarily a binomial operation
    Minus,  // Not necessarily a binomial operation
    Mul,
    Div,
    Mod,
    LShift,
    RShift,
    Not,
    Xor,
    Or,
    And,
    LogicalNot,
    LogicalOr,
    LogicalAnd,
    Equals,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    Increment,
    Decrement,

    // Brackets
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    OpenCodeBracket,
    CloseCodeBracket,

    // Other Syntax
    PointerDeref,
    Semicolon,
    Colon,
    Comma,
    Point,
}

pub struct Lexer<It: Iterator<Item=char>> {
    source: It,
    lookahead: VecDeque<char>
}

impl<It> Lexer<It>
where It: Iterator<Item=char> {
    pub fn new(src: It) -> Lexer<It> {
        Lexer {
            source: src,
            lookahead: VecDeque::new()
        }
    }

    fn next_char(&mut self) -> Option<char> {
        Some(match self.lookahead.pop_front() {
            Some(ch) => ch,
            None => self.source.next()?,
        })
    }
}

impl<It> Iterator for Lexer<It>
where It: Iterator<Item=char> {
    type Item = LexItem;

    fn next(&mut self) -> Option<LexItem> {
        match self.next_char()? {
            '+' => {
                match self.next_char()? {
                    '+' => Some(LexItem::Increment),
                        ch => {
                            self.lookahead.push_front(ch);
                            Some(LexItem::Plus)
                        },
                }
            },
            '-' => Some(LexItem::Minus),
            '*' => Some(LexItem::Mul),
            '/' => Some(LexItem::Div),
            '%' => Some(LexItem::Mod),
            _ => None,  // We have an invalid char
        }
    }
}
