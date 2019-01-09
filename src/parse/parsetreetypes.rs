use crate::lex::constants::LexItem;
use std::rc::Rc;

pub enum ParseNodeType {
    LexItem(LexItem),
    Start,
    TopStatement,
}

pub struct ParseNode {
    node_type: ParseNodeType,
    children: Vec<Rc<ParseNode>>,
}
