use crate::lex::constants::LexItem;
use std::mem::discriminant;
use std::rc::Rc;

pub enum ParseNodeType {
    LexItem(LexItem),
    Start,
    TopStatement,
}

impl ParseNodeType {
    pub fn eq_type(&self, rhs: &ParseNodeType) -> bool {
        if let ParseNodeType::LexItem(item) = self {
            if let ParseNodeType::LexItem(rhs_item) = rhs {
                if let LexItem::Keyword(keyword) = item {
                    if let LexItem::Keyword(rhs_keyword) = rhs_item {
                        discriminant(keyword) == discriminant(rhs_keyword)
                    } else {
                        false
                    }
                } else {
                    discriminant(item) == discriminant(rhs_item)
                }
            } else {
                false
            }
        } else {
            discriminant(self) == discriminant(rhs)
        }
    }
}

pub struct ParseNode {
    node_type: ParseNodeType,
    children: Vec<Rc<ParseNode>>,
    line: usize,
    column: usize,
}
