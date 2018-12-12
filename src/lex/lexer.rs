use std::collections::VecDeque;
use std::iter::Iterator;
use std::str::FromStr;

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

    fn next_chars(&mut self, n: usize) -> Option<String> {
        (0..n).map(|_| self.next_char()).collect::<Option<String>>()
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

            // This means that we shouldn't be treating it as a normal token
            // It only happens if there are no partial matches and we have only one character
            // This will need changing if there are to be normal tokens which include ASCII
            // or Unicode characters valid in identifiers, keywords, or literals
            if too_much && token.len() == 1 {
                self.nextnt_string(&token);
                return None;
            }
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

    fn parse_char_literal(&mut self) -> Option<LexItem> {
        let r = match self.next_char()? {
            '\'' => None,
            '\\' => match self.next_char()? {
                'n' => Some(b'\n'),
                't' => Some(b'\t'),
                'r' => Some(b'\r'),
                '\\' => Some(b'\\'),
                '\'' => Some(b'\''),
                'x' => u8::from_str_radix(&self.next_chars(2)?, 16).ok(),
                _ => unimplemented!(),
            },
            ch => {
                if (ch as u32) <= 0x7f {
                    // This is one byte in utf8
                    Some(ch as u8)
                } else {
                    None
                }
            }
        };
        if self.next_char()? == '\'' {
            Some(LexItem::NumericLiteral(NumberType::UnsignedChar(r?)))
        } else {
            None
        }
    }

    fn parse_type_specifier(&mut self, num: u128) -> Option<NumberType> {
        let mut signed = true;
        let mut size = 32usize;
        while let Some(ch) = self.next_char() {
            match ch.to_ascii_lowercase() {
                'u' => signed = false,
                'l' => size <<= 1,
                'a'...'z' => return None,
                _ => {
                    self.nextnt(ch);
                    break;
                }
            }
        }
        Some(match (size, signed) {
            (8, false) => NumberType::UnsignedChar(num as u8),
            (8, true) => NumberType::SignedChar(num as i8),
            (16, false) => NumberType::UnsignedShort(num as u16),
            (16, true) => NumberType::SignedShort(num as i16),
            (32, false) => NumberType::UnsignedInt(num as u32),
            (32, true) => NumberType::SignedInt(num as i32),
            (64, false) => NumberType::UnsignedLong(num as u64),
            (64, true) => NumberType::SignedLong(num as i64),
            (128, false) => NumberType::UnsignedLongLong(num as u128),
            (128, true) => NumberType::SignedLongLong(num as i128),
            _ => return None,
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
            let mut ch = self.next_after_whitespace()?;
            println!("ch is {}, lookahead is {:?}", ch, self.lookahead);
            match ch {
                '"' => unimplemented!(),
                '0' => {
                    ch = self.next_char()?;
                    match ch {
                        'b' => unimplemented!(),
                        'o' => unimplemented!(),
                        'x' => unimplemented!(),
                        '0'...'9' => unimplemented!(),
                        'U' | 'L' | 'u' | 'l' => {
                            self.nextnt(ch);
                            Some(LexItem::NumericLiteral(self.parse_type_specifier(0)?))
                        }
                        _ => {
                            self.nextnt(ch);
                            Some(LexItem::NumericLiteral(NumberType::SignedInt(0)))
                        }
                    }
                }
                '1'...'9' => {
                    let mut num = String::new();
                    num.push(ch);
                    while let Some(ch) = self.next_char() {
                        if '0' <= ch && ch <= '9' {
                            num.push(ch);
                        } else {
                            self.nextnt(ch);
                            println!("Adding back next char {} to lookahead", ch);
                            break;
                        }
                    }

                    Some(LexItem::NumericLiteral(
                        self.parse_type_specifier(u128::from_str(&num).ok()?)?,
                    ))
                }
                '\'' => self.parse_char_literal(),
                _ => unimplemented!(),
            }
        })
    }
}

#[cfg(test)]
fn test_lexer_str(s: &str, tokens: &[LexItem]) {
    let lexer = Lexer::new(s.chars());

    let vec = lexer.collect::<Vec<_>>();

    println!("got symbols {:?}", vec);

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
    );
}

#[test]
fn test_lexer_valid_char_literal() {
    test_lexer_str(
        "'c' '\\x1b''\\\\'\t\t' '",
        &[
            LexItem::NumericLiteral(NumberType::UnsignedChar(b'c')),
            LexItem::NumericLiteral(NumberType::UnsignedChar(b'\x1b')),
            LexItem::NumericLiteral(NumberType::UnsignedChar(b'\\')),
            LexItem::NumericLiteral(NumberType::UnsignedChar(b' ')),
        ],
    );
}

#[test]
fn test_lexer_invalid_char_literal() {
    test_lexer_str("'", &[]);
    test_lexer_str("''", &[]);
    test_lexer_str("'\\\\", &[]);
}

#[test]
fn test_lexer_int_literal() {
    test_lexer_str(
        "1234,-   0ul 332ll",
        &[
            LexItem::NumericLiteral(NumberType::SignedInt(1234)),
            LexItem::Comma,
            LexItem::Minus,
            LexItem::NumericLiteral(NumberType::UnsignedLong(0)),
            LexItem::NumericLiteral(NumberType::SignedLongLong(332)),
        ],
    )
}
