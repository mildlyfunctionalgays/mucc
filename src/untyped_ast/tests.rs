use crate::lex::Lexer;
use crate::parse::parser::parse;
use crate::untyped_ast::build_untyped_ast;
use crate::untyped_ast::types::Root;
use crate::untyped_ast::types::TopStatement;
use crate::untyped_ast::types::Type;

#[test]
fn test_forward_declaration() {
    let text = "int a(); int b(); \n\n";
    let ast = build_untyped_ast(parse(Lexer::new(text.chars())));
    assert_eq!(
        ast,
        Root(vec![
            TopStatement::ForwardDeclaration(Type::SignedInt, "a".to_string(), Vec::new()),
            TopStatement::ForwardDeclaration(Type::SignedInt, "b".to_string(), Vec::new()),
        ])
    )
}
