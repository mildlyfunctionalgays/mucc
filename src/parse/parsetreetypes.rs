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

#[derive(Debug, PartialEq, Clone)]
pub enum NonTerminalType {
    Start, // There must only be one Start rule
    TopStatements,
    TopStatement,
    FunctionPointer,
    Type,
    Pointers,
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
    Return,
    Expression,
    ExpressionWithoutComma,
    Expression15,
    Expression15Trail,
    Expression14,
    Operator14,
    Expression13,
    Expression12,
    Expression12Trail,
    Expression11,
    Expression11Trail,
    Expression10,
    Expression10Trail,
    Expression9,
    Expression9Trail,
    Expression8,
    Expression8Trail,
    Expression7,
    Expression7Trail,
    Expression6,
    Expression6Trail,
    Expression5,
    Expression5Trail,
    Expression4,
    Expression4Trail,
    Expression3,
    Expression3Trail,
    Expression2,
    Expression1,
    Expression1NoTrail,
    Expression1Trail,
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
