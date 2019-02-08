use crate::parse::parsetreetypes::NonTerminalType;
use crate::parse::parsetreetypes::ParseNode;
use crate::parse::parsetreetypes::ParseNodeType;
use crate::untyped_ast::top_statement::read_top_statements;
use crate::untyped_ast::types::Root;
use std::rc::Rc;

pub fn build_untyped_ast(node: Rc<ParseNode>) -> Root {
    require_non_terminal!(node, NonTerminalType::Start);
    require_len!(node, |len| len == 1);

    Root(read_top_statements(node.children[0].clone()))
}
