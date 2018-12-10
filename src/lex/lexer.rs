enum LexItem {
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

struct Lexer<It: Iterator<char>> {
    source: It,
    lookahead: VecDequeue<char>
}

impl<It> Lexer<It> {
    pub fn new(&mut src: It) -> Lexer<It> {
        Lexer {
            source: src,
            lookahead: VecDequeue::new()
        }
    }

}

impl<It> Iterator<Item = LexItem> for Lexer<It> {
    fn next(&mut self) -> Option<Self::Item> {
        let mut chr = self.source.next()?;
        match chr {
            '+' => Some(Plus),
            '-' => Some(Minus),
            '*' => Some(Mul),
            '/' => Some(Div),
            '%' => Some(Mod),
            _ => None,
        }
    }
}