use crate::lex::constants::{LexItem, LexKeyword, NumberType};
use crate::parse::parsetreetypes::ParseNodeType;

#[test]
fn test_parse_node_eq_keyword() {
    let t = ParseNodeType::LexItem(LexItem::Keyword(LexKeyword::Char));
    assert!(t.eq_type(&t))
}

#[test]
fn test_parse_node_not_eq_keyword() {
    let t = ParseNodeType::LexItem(LexItem::Keyword(LexKeyword::Char));
    let t2 = ParseNodeType::LexItem(LexItem::Keyword(LexKeyword::Case));
    assert!(!t.eq_type(&t2))
}

#[test]
fn test_parse_node_eq_start() {
    let t = ParseNodeType::Start;
    assert!(t.eq_type(&t))
}

#[test]
fn test_parse_node_not_eq_differing_levels() {
    let t = ParseNodeType::Start;
    let t2 = ParseNodeType::LexItem(LexItem::Keyword(LexKeyword::Case));
    assert!(!t.eq_type(&t2))
}

#[test]
fn test_parse_node_eq_discriminant() {
    let t = ParseNodeType::LexItem(LexItem::NumericLiteral(NumberType::UnsignedInt(42)));
    let t2 = ParseNodeType::LexItem(LexItem::NumericLiteral(NumberType::UnsignedChar(2)));
    assert!(t.eq_type(&t2))
}
