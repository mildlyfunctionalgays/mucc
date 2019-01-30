use crate::lex::lexer::Lexer;
use crate::parse::parser;
use crate::parse::parser::parse;

#[test]
fn test_parse_empty() {
    let text = "\n";
    let parse = parse(Lexer::new(text.chars()));
}

#[test] #[ignore]
fn test_parse_semicolon() {
    let text = ";";
    let parse = parse(Lexer::new(text.chars()));
}

#[test] #[ignore]
fn test_parse_minimal_main() {
    let text = "int main() {\n    return 0;\n}\n";
    let parse = parse(Lexer::new(text.chars()));
}