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
pub enum NumberType {
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

pub const LITERAL_TOKENS: &[(&str, LexItem)] = &[
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
    ("while\x00", LexItem::Keyword(LexKeyword::While)),
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
