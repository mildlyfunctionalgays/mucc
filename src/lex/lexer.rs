use std::collections::VecDeque;
use std::iter::Iterator;

#[derive(Debug)]
pub enum LexKeyword {
    Struct,
    Typedef,
    If,
    For,
    While,
    Do,
}

#[derive(Debug)]
pub enum LexItem {
    // Literals
    StringLiteral(String),
    NumericLiteral(String), // only return a string here so we can figure out the type later
    FloatLiteral(String),
    HexLiteral(String),

    Identifier(String),
    Keyword(LexKeyword),

    // Operations
    Plus,  // Not necessarily a binomial operation
    Minus, // Not necessarily a binomial operation
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
    LeftCurlyBrace,
    RightCurlyBrace,

    // Other Syntax
    PointerDeref,
    Semicolon,
    Colon,
    Comma,
    Period,
}

pub struct Lexer<It: Iterator<Item = char>> {
    source: It,
    lookahead: VecDeque<char>,
}

impl<It> Lexer<It>
where
    It: Iterator<Item = char>,
{
    pub fn new(src: It) -> Lexer<It> {
        Lexer {
            source: src,
            lookahead: VecDeque::new(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.lookahead.pop_back().or_else(|| self.source.next())
    }

    fn skip_chars(&mut self, chars: &str) -> Option<char> {
        loop {
            let ch = self.next_char()?;
            if !chars.chars().any(|c| c == ch) {
                break Some(ch);
            }
        }
    }
    fn next_after_whitespace(&mut self) -> Option<char> {
        self.skip_chars(" \n\t\r")
    }


    fn is_valid_identifier_start(&self, ch: char) -> bool {
        unimplemented!()
    }

    fn is_valid_identifier(&self, ch: char) -> bool {
        unimplemented!()
    }
    fn nextnt(&mut self, ch: char) {
        self.lookahead.push_back(ch);
    }
}

impl<It> Iterator for Lexer<It>
where
    It: Iterator<Item = char>,
{
    type Item = LexItem;

    fn next(&mut self) -> Option<LexItem> {
        Some(match self.next_after_whitespace()? {
            '+' =>
                // Use next_char here because you aren't supposed to accept + +
                match self.next_char()? {
                    '+' => LexItem::Increment,
                    ch => {
                        self.nextnt(ch);
                        LexItem::Plus
                    },
                }
            '-' => match self.next_char()? {
                '-' => LexItem::Decrement,
                '>' => LexItem::PointerDeref,
                ch => {
                    self.nextnt(ch);
                    LexItem::Minus
                },
            },

            '*' => LexItem::Mul,
            '/' => LexItem::Div,
            '%' => LexItem::Mod,
            ch => return None,
        })
    }
}
