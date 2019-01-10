use crate::lex::constants::LexItem;
use crate::lex::constants::LexKeyword;
use crate::lex::constants::LITERAL_TOKENS;
use crate::lex::errors::LexSuccess;
use std::mem::discriminant;
use std::mem::Discriminant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseNodeType {
    Lex(Discriminant<LexItem>),
    Keyword(Discriminant<LexKeyword>),
    RawLex(LexItem),
    Start, // There must only be one Start rule
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
            panic!()
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

impl From<LexSuccess> for ParseNodeType {
    fn from(value: LexSuccess) -> Self {
        if let LexItem::Keyword(kw) = value.item {
            ParseNodeType::Keyword(discriminant(&kw))
        } else {
            ParseNodeType::Lex(discriminant(&value.item))
        }
    }
}

#[derive(Clone)]
pub struct ParseNode {
    pub node_type: ParseNodeType,
    pub children: Vec<Rc<ParseNode>>,
}
