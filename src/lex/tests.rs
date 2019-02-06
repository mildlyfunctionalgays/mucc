use super::types::{LexItem, NumberType};
use super::Lexer;
use crate::lex::errors::LexErrorType;

#[cfg(test)]
fn test_lexer_str(s: &str, tokens: &[LexItem]) {
    let lexer = Lexer::new(s.chars());

    let vec = lexer.map(|res| res.unwrap().item).collect::<Vec<_>>();

    println!("got symbols {:?}", vec);

    assert_eq!(vec.as_slice(), tokens);
}

#[cfg(test)]
fn test_lexer_str_error(s: &str, tokens: &[Result<LexItem, LexErrorType>]) {
    let lexer = Lexer::new(s.chars());

    let vec = lexer
        .map(|res| {
            res.map(|success| success.item)
                .map_err(|err| err.error_type)
        })
        .collect::<Vec<_>>();

    println!("got symbols {:?}", vec);

    assert_eq!(vec.as_slice(), tokens);
}

#[cfg(test)]
fn test_lexer_str_first_error(s: &str, tokens: Result<&[LexItem], &LexErrorType>) {
    let lexer = Lexer::new(s.chars());

    let result = lexer
        .map(|res| {
            res.map(|success| success.item)
                .map_err(|err| err.error_type)
        })
        .collect::<Result<Vec<_>, _>>();
    let result = result.as_ref().map(|v| v.as_slice());

    println!("got symbols {:?}", result);

    assert_eq!(result, tokens);
}

#[test]
fn test_lexer_error_invalid_size_literal() {
    test_lexer_str_error("0ulll", &[Err(LexErrorType::InvalidSize(256))]);
}

#[test]
fn test_lexer_zero() {
    test_lexer_str("0", &[LexItem::NumericLiteral(NumberType::SignedInt(0))])
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
            LexItem::NumericLiteral(NumberType::UnsignedInt(u32::from(b'c'))),
            LexItem::NumericLiteral(NumberType::UnsignedInt(u32::from(b'\x1b'))),
            LexItem::NumericLiteral(NumberType::UnsignedInt(u32::from(b'\\'))),
            LexItem::NumericLiteral(NumberType::UnsignedInt(u32::from(b' '))),
        ],
    );
}

#[test]
fn test_lexer_while_keyword() {
    test_lexer_str("while", &[LexItem::While]);
    test_lexer_str("while[", &[LexItem::While, LexItem::LeftBracket]);
}

#[test]
fn test_lexer_invalid_char_literal() {
    test_lexer_str_error("'", &[Err(LexErrorType::Unfinished("'".to_string()))]);
    test_lexer_str_error("''", &[Err(LexErrorType::InvalidLiteral("''".to_string()))]);
    test_lexer_str_error("'\\\\", &[Err(LexErrorType::Unfinished("'\\".to_string()))]);
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
            LexItem::NumericLiteral(NumberType::SignedInt(0b10_1011)),
            LexItem::Comma,
            LexItem::Minus,
            LexItem::NumericLiteral(NumberType::UnsignedLong(0o70)),
            LexItem::NumericLiteral(NumberType::SignedLongLong(0x12f)),
            LexItem::Plus,
            LexItem::NumericLiteral(NumberType::UnsignedLong(0xdead_beef)),
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
fn test_lexer_function_definition() {
    test_lexer_str(
        "int main(int argc, char *argv[]) {\n\treturn 0;\n}",
        &[
            LexItem::Int,
            LexItem::Identifier("main".to_string()),
            LexItem::LeftParen,
            LexItem::Int,
            LexItem::Identifier("argc".to_string()),
            LexItem::Comma,
            LexItem::Char,
            LexItem::Mul,
            LexItem::Identifier("argv".to_string()),
            LexItem::LeftBracket,
            LexItem::RightBracket,
            LexItem::RightParen,
            LexItem::LeftCurlyBrace,
            LexItem::Return,
            LexItem::NumericLiteral(NumberType::SignedInt(0)),
            LexItem::Semicolon,
            LexItem::RightCurlyBrace,
        ],
    )
}

#[test]
fn test_lexer_string_literal() {
    test_lexer_str(
        "\"Hello,\t world\",\'c\' \" This is a ☭ \\\" test \\\"\" ",
        &[
            LexItem::StringLiteral(b"Hello,\t world".to_vec()),
            LexItem::Comma,
            LexItem::NumericLiteral(NumberType::UnsignedInt(u32::from(b'c'))),
            LexItem::StringLiteral(" This is a ☭ \" test \"".as_bytes().to_vec()),
        ],
    )
}

#[test]
fn test_lexer_location() {
    let s = "int main(int argc, char *argv[]) {\n\treturn 0;\n}";
    let lexer = Lexer::new(s.chars());
    let vec = lexer
        .map(|res| {
            res.map(|success| (success.line, success.column))
                .unwrap_or_else(|err| (err.location.line, err.location.column))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![
            (1, 1),
            (1, 5),
            (1, 9),
            (1, 10),
            (1, 14),
            (1, 18),
            (1, 20),
            (1, 25),
            (1, 26),
            (1, 30),
            (1, 31),
            (1, 32),
            (1, 34),
            (2, 2),
            (2, 9),
            (2, 10),
            (3, 1),
        ]
    );
}

#[test]
fn test_lexer_comment() {
    test_lexer_str("//hi!!", &[]);
}

#[cfg(test)]
#[allow(dead_code)]
fn test_no_panic(text: &str) {
    Lexer::new(text.chars()).for_each(drop);
}

#[test]
fn test_invalid_escape() {
    test_lexer_str_first_error(
        r#""\,""#,
        Err(&LexErrorType::InvalidEscape(",".to_string())),
    );
}

#[test]
fn test_lower_unicode_escape() {
    test_lexer_str(r#""\u0000""#, &[LexItem::StringLiteral(vec![0])]);
}

#[test]
fn test_invalid_decimal_escape() {
    test_lexer_str_first_error(
        r#""\0n  ""#,
        Err(&LexErrorType::InvalidEscape("0n  ".to_string())),
    );
}

#[test]
fn test_unclosed_string() {
    test_lexer_str_first_error(
        r#"""#,
        Err(&LexErrorType::UnclosedStringLiteral("".to_string())),
    );
    test_lexer_str_first_error(
        r#""some text
    ""#,
        Err(&LexErrorType::UnclosedStringLiteral(
            "some text".to_string(),
        )),
    );
}

#[test]
fn test_incomplete_binary_literal() {
    test_lexer_str_first_error("0b", Err(&LexErrorType::EmptyNumericLiteral));
}

#[test]
fn test_zero_at_end() {
    test_lexer_str("0", &[LexItem::NumericLiteral(NumberType::SignedInt(0))]);
}
