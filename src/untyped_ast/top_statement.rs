use crate::parse::parsetreetypes::NonTerminalType;
use crate::parse::parsetreetypes::ParseNode;
use crate::parse::parsetreetypes::ParseNodeType;
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

    unimplemented!()
}
