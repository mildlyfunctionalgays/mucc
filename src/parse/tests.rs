use crate::lex::lexer::Lexer;
use crate::parse::parser;

#[test] #[ignore]
fn test_parse_minimal_main() {
    let text = "int main() {\n    return 0;\n}\n";
    let tokens = Lexer::new(text.chars());
    let parse = parser::parse(tokens);

    // TODO finish test
}