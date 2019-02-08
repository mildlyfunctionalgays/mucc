use crate::lex::types::LexItem;
use crate::parse::types::NonTerminalType;
use crate::parse::types::ParseNode;
use crate::parse::types::ParseNodeType;
use crate::untyped_ast::types::TopStatement;
use std::rc::Rc;

pub(super) fn read_top_statements(node: Rc<ParseNode>) -> Vec<TopStatement> {
    require_non_terminal!(node, NonTerminalType::TopStatements);

    let mut statements = Vec::new();

    for child in node.children.clone() {
        match child.node_type {
            ParseNodeType::NonTerminal(NonTerminalType::TopStatement) => {
                statements.push(read_top_statement(child))
            }
            ParseNodeType::NonTerminal(NonTerminalType::TopStatements) => {
                statements.extend(read_top_statements(child).into_iter())
            }
            _ => unreachable!(),
        }
    }

    statements
}

pub(super) fn read_top_statement(node: Rc<ParseNode>) -> TopStatement {
    require_non_terminal!(node, NonTerminalType::TopStatement);
    require_len!(node, |len| len == 1);

    match node.children[0].node_type {
        ParseNodeType::NonTerminal(NonTerminalType::Declaration) => unimplemented!(),
        ParseNodeType::NonTerminal(NonTerminalType::ForwardDeclaration) => {
            read_forward_declaration(node)
        }
        ParseNodeType::NonTerminal(NonTerminalType::FunctionDeclaration) => {
            read_function_declaration(node)
        }
        ParseNodeType::NonTerminal(NonTerminalType::StructOrUnionDeclaration) => unimplemented!(),
        ParseNodeType::NonTerminal(NonTerminalType::Typedef) => unimplemented!(),
        _ => unreachable!(),
    }
}

fn read_forward_declaration(node: Rc<ParseNode>) -> TopStatement {
    require_non_terminal!(node, NonTerminalType::ForwardDeclaration);
    require_len!(node, |len| len == 2);
    require_terminal!(node, 1, LexItem::Semicolon);

    unimplemented!()
}

fn read_function_declaration(node: Rc<ParseNode>) -> TopStatement {
    require_non_terminal!(node, NonTerminalType::FunctionDeclaration);
    require_len!(node, |len| len == 2);

    unimplemented!()
}
