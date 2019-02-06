#[derive(Clone, Debug, PartialEq)]
pub enum NumberType {
    #[allow(dead_code)]
    Float(f32),
    #[allow(dead_code)]
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

impl Default for NumberType {
    fn default() -> Self {
        NumberType::UnsignedChar(0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexItem {
    // Literals
    StringLiteral(Vec<u8>),
    NumericLiteral(NumberType),

    Identifier(String),

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

    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LShiftAssign,
    RShiftAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    Assign,

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
    Question,

    // Keywords
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
    Bool,
    Complex,
    Imaginary,
}
