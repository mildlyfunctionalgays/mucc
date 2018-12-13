use super::constants::{LexItem, NumberType};
use super::lexer::Lexer;
use crate::lex::constants::LexKeyword;

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
            LexItem::NumericLiteral(NumberType::UnsignedInt(b'c' as u32)),
            LexItem::NumericLiteral(NumberType::UnsignedInt(b'\x1b' as u32)),
            LexItem::NumericLiteral(NumberType::UnsignedInt(b'\\' as u32)),
            LexItem::NumericLiteral(NumberType::UnsignedInt(b' ' as u32)),
        ],
    );
}

#[test]
fn test_lexer_while_keyword() {
    test_lexer_str("while", &[LexItem::Keyword(LexKeyword::While)]);
    test_lexer_str(
        "while[",
        &[LexItem::Keyword(LexKeyword::While), LexItem::LeftBracket],
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

#[test]
fn test_lexer_nonint_literal() {
    test_lexer_str(
        "0b101011,-0o70ul 0x12fll+0xDeAdBeEfuL 69l 0105u",
        &[
            LexItem::NumericLiteral(NumberType::SignedInt(0b101011)),
            LexItem::Comma,
            LexItem::Minus,
            LexItem::NumericLiteral(NumberType::UnsignedLong(0o70)),
            LexItem::NumericLiteral(NumberType::SignedLongLong(0x12f)),
            LexItem::Plus,
            LexItem::NumericLiteral(NumberType::UnsignedLong(0xdeadbeef)),
            LexItem::NumericLiteral(NumberType::SignedLong(69)),
            LexItem::NumericLiteral(NumberType::UnsignedInt(0o105)),
        ],
    )
}
#[test]
fn test_lexer_identifier() {
    test_lexer_str(
        "[hello](var1able,a, ⚧, 3u nit)",
        &[
            LexItem::LeftBracket,
            LexItem::Identifier("hello".to_string()),
            LexItem::RightBracket,
            LexItem::LeftParen,
            LexItem::Identifier("var1able".to_string()),
            LexItem::Comma,
            LexItem::Identifier("a".to_string()),
            LexItem::Comma,
            LexItem::Identifier("⚧".to_string()),
            LexItem::Comma,
            LexItem::NumericLiteral(NumberType::UnsignedInt(3)),
            LexItem::Identifier("nit".to_string()),
            LexItem::RightParen,
        ],
    )
}

#[test]
fn test_lexer_funcion_definition() {
    test_lexer_str(
        "int main(int argc, char *argv[]) {\n\treturn 0;\n}",
        &[
            LexItem::Keyword(LexKeyword::Int),
            LexItem::Identifier("main".to_string()),
            LexItem::LeftParen,
            LexItem::Keyword(LexKeyword::Int),
            LexItem::Identifier("argc".to_string()),
            LexItem::Comma,
            LexItem::Keyword(LexKeyword::Char),
            LexItem::Mul,
            LexItem::Identifier("argv".to_string()),
            LexItem::LeftBracket,
            LexItem::RightBracket,
            LexItem::RightParen,
            LexItem::LeftCurlyBrace,
            LexItem::Keyword(LexKeyword::Return),
            LexItem::NumericLiteral(NumberType::SignedInt(0)),
            LexItem::Semicolon,
            LexItem::RightCurlyBrace,
        ],
    )
}
