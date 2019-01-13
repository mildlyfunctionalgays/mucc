use crate::lex;
use crate::lex::constants::LexItem;
use crate::lex::constants::LexKeyword;
use crate::lex::constants::NumberType;
use crate::lex::constants::LITERAL_TOKENS;
use crate::lex::lexer::Lexer;
use std::mem::discriminant;
use std::mem::Discriminant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseNodeType {
    Lex(Discriminant<LexItem>),
    Keyword(Discriminant<LexKeyword>),
    Start,
    TopStatement,
    FunctionPointer,
    Type,
    BasicType,
    TypeSpecifier,
    MaybeType,
    BasicDeclaration,
    FunctionDeclaration,
    ForwardDeclaration,
    StructOrUnionDeclaration,
    StructOrUnion,
    MaybeIdentifier,
    StructMembers,
    StructMember,
    MaybeBitfield,
    Block,
    BlockContents,
    Statement,
    Declaration,
    ExpressionOrDeclaration,
    ForLoop,
    WhileLoop,
    DoWhileLoop,
    Expression,
    ExpressionWithoutComma,
    Expression15,
    Expression14,
    Operator14,
    Expression13,
    Expression12,
    Expression11,
    Expression10,
    Expression9,
    Expression8,
    Expression7,
    Expression6,
    Expression5,
    Expression4,
    Expression3,
    Expression2,
    Expression1,
    Operator7,
    Operator6,
    Operator5,
    Operator4,
    Operator3,
    Expression2_5,
    Operator2,
    UnaryOperator1,
    StructOperator,
    CallArguments,
    NonEmptyCallArguments,
    Literal,
    Typedef,
    If,
    TypeWithoutIdentifier,
    Args,
    Members,
}

//impl From<fn(String) -> LexItem> for ParseNodeType {
//    fn from(value: fn(String) -> LexItem) -> Self {
//        ParseNodeType::Lex(discriminant(value.call((String::default(),))))
//    }
//}

//impl<T: Fn(NumberType) -> LexItem> From<T> for ParseNodeType {
//    fn from(value: T) -> Self {
//        ParseNodeType::Lex(discriminant(value.call((NumberType::default(),))))
//    }
//}

impl From<&str> for ParseNodeType {
    fn from(value: &str) -> Self {
        let match_: Option<&(&str, LexItem)> = LITERAL_TOKENS
            .iter()
            .find(|(key, _)| key.trim_end_matches('\x00') == value);
        if let Some(match_) = match_ {
            if let LexItem::Keyword(keyword) = &match_.1 {
                ParseNodeType::Keyword(discriminant(keyword))
            } else {
                ParseNodeType::Lex(discriminant(&match_.1))
            }
        } else {
            panic!(format!(r#"The string "{}" does not match a token"#, value))
        }
    }
}

impl From<Discriminant<LexItem>> for ParseNodeType {
    fn from(value: Discriminant<LexItem>) -> Self {
        ParseNodeType::Lex(value)
    }
}

impl From<Discriminant<LexKeyword>> for ParseNodeType {
    fn from(value: Discriminant<LexKeyword>) -> Self {
        ParseNodeType::Keyword(value)
    }
}

/*impl From<ParseNodeType> for ParseNodeType {
    fn from(value: ParseNodeType) -> Self {
        value
    }
}*/

pub struct ParseNode {
    node_type: ParseNodeType,
    children: Vec<Rc<ParseNode>>,
    line: usize,
    column: usize,
}
