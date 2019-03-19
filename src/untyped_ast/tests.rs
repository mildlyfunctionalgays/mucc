use crate::lex::Lexer;
use crate::parse::parser::parse;
use crate::untyped_ast::build_untyped_ast;
use crate::untyped_ast::types::TopStatement;
use crate::untyped_ast::types::Type;
use crate::untyped_ast::types::{BaseType, Root};

#[test]
fn test_forward_declaration() {
    let text = "int a(int arg); int b(int, int a2); \n\n";
    let ast = build_untyped_ast(parse(Lexer::new(text.chars())).unwrap());
    assert_eq!(
        ast,
        Root(vec![
            TopStatement::ForwardDeclaration(
                Type::new(BaseType::SignedInt),
                "a".to_string(),
                vec![Type::new(BaseType::SignedInt)]
            ),
            TopStatement::ForwardDeclaration(
                Type::new(BaseType::SignedInt),
                "b".to_string(),
                vec![
                    Type::new(BaseType::SignedInt),
                    Type::new(BaseType::SignedInt)
                ]
            ),
        ])
    )
}
