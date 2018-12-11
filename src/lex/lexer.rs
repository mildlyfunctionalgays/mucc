use std::collections::VecDeque;
use std::iter::Iterator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexKeyword {
    Struct,
    Typedef,
    If,
    For,
    While,
    Do,
}

#[derive(Clone, Debug, PartialEq)]
enum NumberType {
    Float(f32),
    Double(f64),
    SignedChar(i8),
    UnsignedChar(u8),
    SignedShort(i16),
    UnsignedShort(u16),
    SignedInt(i32),
    UnsignedInt(u32),
    SignedLong(i64),
    UnsignedLong(u64),
    SignedLongLong(i128),
    UnsignedLongLong(u128),
}

const LITERAL_TOKENS: &[(&str, LexItem)] = &[
    ("<=", LexItem::LessOrEqual),
    ("==", LexItem::Equals),
    ("!=", LexItem::NotEqual),
    (">=", LexItem::GreaterOrEqual),
    (">", LexItem::GreaterThan),
    ("+", LexItem::Plus),
    ("-", LexItem::Minus),
    ("*", LexItem::Mul),
    ("/", LexItem::Div),
    ("%", LexItem::Mod),
    ("<<", LexItem::LShift),
    (">>", LexItem::RShift),
    ("~", LexItem::Not),
    ("^", LexItem::Xor),
    ("|", LexItem::Or),
    ("&", LexItem::And),
    ("!", LexItem::LogicalNot),
    ("||", LexItem::LogicalOr),
    ("&&", LexItem::LogicalAnd),
    ("==", LexItem::Equals),
    ("!=", LexItem::NotEqual),
    ("<", LexItem::LessThan),
    (">", LexItem::GreaterThan),
    ("<=", LexItem::LessOrEqual),
    (">=", LexItem::GreaterOrEqual),
    ("++", LexItem::Increment),
    ("--", LexItem::Decrement),
    ("(", LexItem::LeftParen),
    (")", LexItem::RightParen),
    ("[", LexItem::LeftBracket),
    ("]", LexItem::RightBracket),
    ("{", LexItem::LeftCurlyBrace),
    ("}", LexItem::RightCurlyBrace),
    ("->", LexItem::PointerDeref),
    (";", LexItem::Semicolon),
    (":", LexItem::Colon),
    (",", LexItem::Comma),
    (".", LexItem::Period),
];

#[derive(Clone, Debug, PartialEq)]
pub enum LexItem {
    // Literals
    StringLiteral(Vec<u8>),
    NumericLiteral(NumberType),
    FloatLiteral(String),

    Identifier(String),
    Keyword(LexKeyword),

    // Operations
    Plus,
    // Not necessarily a binomial operation
    Minus,
    // Not necessarily a binomial operation
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

    fn nextnt_string(&mut self, s: &str) {
        self.lookahead.extend(s.chars().rev());
    }

    fn next_regular_token(&mut self) -> Option<LexItem> {
        let mut token: String = self.next_after_whitespace()?.to_string();

        Some(loop {
            let exact_match = LITERAL_TOKENS
                .iter()
                .find(|(key, _)| token == *key)
                .map(|(_, value)| value);
            let partial_matches: Vec<&LexItem> = LITERAL_TOKENS
                .iter()
                .filter_map(|(key, val)| {
                    if key.starts_with(&token) {
                        Some(val)
                    } else {
                        None
                    }
                })
                .collect();

            println!("{:?}: {:?}", token, partial_matches);

            let mut too_much = match partial_matches.len() {
                1 => {
                    if let Some(match_) = exact_match {
                        break match_.clone();
                    } else {
                        false
                    }
                }
                0 => true,
                _ => false,
            };
            if !too_much {
                if let Some(char) = self.next_char() {
                    token.push(char);
                } else {
                    too_much = true;
                }
            }
            if too_much {
                let largest_match = LITERAL_TOKENS
                    .iter()
                    .filter(|(key, _)| token.starts_with(key))
                    .max_by_key(|(key, _)| key.len());
                let (key, value) = largest_match?;
                self.nextnt_string(&token[key.len()..]);
                break value.clone();
            }
        })
    }
}

impl<It> Iterator for Lexer<It>
where
    It: Iterator<Item = char>,
{
    type Item = LexItem;

    fn next(&mut self) -> Option<LexItem> {
        self.next_regular_token().or_else(|| {
            let ch = self.next_after_whitespace()?;
            match ch {
                '"' => unimplemented!(),
                '0'...'9' => {
                    if ch == '0' {
                        unimplemented!()
                    } else {
                        unimplemented!()
                    }
                }
                '\'' => unimplemented!(),
                _ => unimplemented!(),
            }
        })
    }
}

#[cfg(test)]
fn test_lexer_str(s: &str, tokens: &[LexItem]) {
    let lexer = Lexer::new(s.chars());

    let vec = lexer.collect::<Vec<_>>();

    assert_eq!(vec.as_slice(), tokens);
}

#[test]
fn test_lexer_plus() {
    test_lexer_str("+", &[LexItem::Plus]);
}

#[test]
fn test_lexer_increment() {
    test_lexer_str("++", &[LexItem::Increment]);
}

#[test]
fn test_lexer_plus_plus() {
    test_lexer_str("+ +", &[LexItem::Plus, LexItem::Plus]);
}

#[test]
fn test_lexer_pointerderef_lessthan_minus() {
    test_lexer_str(
        "-><-",
        &[LexItem::PointerDeref, LexItem::LessThan, LexItem::Minus],
    );
}

#[test]
fn test_lexer_triple() {
    test_lexer_str(
        "|||&&|",
        &[
            LexItem::LogicalOr,
            LexItem::Or,
            LexItem::LogicalAnd,
            LexItem::Or,
        ],
    )
}
