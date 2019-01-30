use crate::lex::constants::LexItem;
use crate::lex::constants::LITERAL_TOKENS;
use crate::lex::errors::LexSuccess;
use std::mem::discriminant;
use std::mem::Discriminant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseNodeType {
    Lex(Discriminant<LexItem>),
    RawLex(LexSuccess),
    Start, // There must only be one Start rule
    TopStatements,
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
            ParseNodeType::Lex(discriminant(&match_.1))
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

impl From<LexSuccess> for ParseNodeType {
    fn from(value: LexSuccess) -> Self {
        ParseNodeType::Lex(discriminant(&value.item))
    }
}

#[derive(Clone, Debug)]
pub struct ParseNode {
    pub node_type: ParseNodeType,
    pub children: Vec<Rc<ParseNode>>,
}

impl ParseNode {
    pub fn from_lex(lex: LexSuccess) -> Self {
        ParseNode {
            node_type: ParseNodeType::RawLex(lex),
            children: Vec::new(),
        }
    }
}
