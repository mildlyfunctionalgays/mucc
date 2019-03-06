use crate::lex::constants::LITERAL_TOKENS;
use crate::lex::errors::LexSuccess;
use crate::lex::types::LexItem;
use std::mem::discriminant;
use std::mem::Discriminant;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum RuleType {
    Terminal(Discriminant<LexItem>),
    NonTerminal(NonTerminalType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseNodeType {
    Terminal(LexSuccess),
    NonTerminal(NonTerminalType),
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum NonTerminalType {
    Start, // There must only be one Start rule
    TopStatements,
    TopStatement,
    FunctionPointer,
    Type,
    BasicType,
    TypeSpecifier,
    MaybeType,
    TypeWithIdentifier,
    TypeWithMaybeIdentifier,
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
    Return,
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
    MaybeElse,
    TypeWithoutIdentifier,
    Args,
    Members,
}

impl From<&str> for RuleType {
    fn from(value: &str) -> Self {
        let match_: Option<&(&str, LexItem)> = LITERAL_TOKENS
            .iter()
            .find(|(key, _)| key.trim_end_matches('\x00') == value);
        if let Some(match_) = match_ {
            RuleType::Terminal(discriminant(&match_.1))
        } else {
            panic!(format!(r#"The string "{}" does not match a token"#, value))
        }
    }
}

impl From<Discriminant<LexItem>> for RuleType {
    fn from(value: Discriminant<LexItem>) -> Self {
        RuleType::Terminal(value)
    }
}

impl From<LexSuccess> for RuleType {
    fn from(value: LexSuccess) -> Self {
        RuleType::Terminal(discriminant(&value.item))
    }
}

impl From<LexSuccess> for ParseNodeType {
    fn from(value: LexSuccess) -> Self {
        ParseNodeType::Terminal(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParseNode {
    pub node_type: ParseNodeType,
    pub children: Vec<Rc<ParseNode>>,
}
