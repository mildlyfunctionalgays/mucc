#![allow(unused_variables, unused_imports)]
use super::parsetreetypes::{ParseNode, ParseNodeType};
use super::rules::RULES;
use crate::lex::errors::{LexError, LexResult};

pub fn parse<T: Iterator<Item = LexResult>>(tokens: T) -> ParseNode {
    unimplemented!()
}
