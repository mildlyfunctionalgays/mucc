use crate::lex::types::{LexItem, NumberType};
use crate::parse::types::{
    NonTerminalType::{self, *},
    RuleType,
};
use lazy_static::lazy_static;
use std::mem::discriminant;

macro_rules! grammar {
    (@token StringLiteral) => {
        RuleType::Terminal(discriminant(&LexItem::StringLiteral(Vec::new())))
    };
    (@token StringLiteral) => {
        RuleType::Terminal(discriminant(&LexItem::StringLiteral(Vec::new())))
    };
    (@token Identifier) => {
        RuleType::Terminal(discriminant(&LexItem::Identifier(String::new())))
    };
    (@token NumericLiteral) => {
        RuleType::Terminal(discriminant(&LexItem::NumericLiteral(NumberType::default())))
    };
    (@token $token:ident) => {
        RuleType::NonTerminal($token)
    };
    (@token $token:tt) => {
        {
            RuleType::from($token)
        }
    };
    (@beginline @eof $($tail:tt)*) => {
        grammar!(@eof $($tail)*)
    };
    (@beginline $token:ident -> $($tail:tt)*) => {
        grammar!(@replacement ($token, vec![]) $($tail)*)
    };
    (@replacement $parens:tt , $($tail:tt)*) => {
        grammar!(@beginline $($tail)* $parens)
    };
    (@replacement $parens:tt ε $($tail:tt)*) => {
        grammar!(@replacement $parens $($tail)*)
    };
    (@replacement ($name:tt, vec![$($expression:tt)*]) $token:tt $($tail:tt)*) => {
        grammar!(@replacement ($name, vec![$($expression)* grammar!(@token $token), ]) $($tail)*)
    };
    (@eof $($tail:tt)*) => {
        vec![$($tail),*]
    };
    ($($tail:tt)*) => {
        grammar!(@beginline $($tail)* @eof)
    };
}

lazy_static! {
    pub static ref RULES: &'static [(NonTerminalType, &'static [RuleType])] = &*RULE_VEC_2;
    static ref RULE_VEC_1: Vec<(NonTerminalType, Vec<RuleType>)> = get_rules();
    static ref RULE_VEC_2: Vec<(NonTerminalType, &'static [RuleType])> = {
        (*RULE_VEC_1)
            .iter()
            .map(|&(ref key, ref value)| (key.clone(), value.as_slice()))
            .collect()
    };
}

pub fn get_rules() -> Vec<(NonTerminalType, Vec<RuleType>)> {
    grammar!(
        Start -> TopStatements,
        TopStatements -> ε,
        TopStatements -> TopStatement TopStatements,

        TopStatement -> Declaration ";",
        TopStatement -> Typedef,
        TopStatement -> FunctionDeclaration,
        TopStatement -> ForwardDeclaration,
        TopStatement -> ";",

        Typedef -> "typedef" MaybeType Identifier ";",
        FunctionPointer -> TypeWithoutIdentifier "(",
        Type -> Type "*",
        Type -> TypeQualifier Type,
        Type -> Identifier,
        Type -> TypeSpecifier NumberType,
        NumberType -> TypeSpecifier NumberType,
        NumberType -> TypeQualifier NumberType,
        NumberType -> ε,
        TypeSpecifier -> "int",
        TypeSpecifier -> "long",
        TypeSpecifier -> "float",
        TypeSpecifier -> "short",
        TypeSpecifier -> "char",
        TypeSpecifier -> "double",
        TypeSpecifier -> "unsigned",
        TypeSpecifier -> "signed",
        TypeQualifier -> "register",
        TypeQualifier -> "volatile",
        TypeSpecifier -> StructOrUnionDeclaration,
        MaybeType -> ε,
        MaybeType -> Type,

        TypeWithIdentifier -> Type Identifier,
        // TODO: Add function pointer support

        TypeWithMaybeIdentifier -> TypeWithIdentifier,
        TypeWithMaybeIdentifier -> Type,

        BasicDeclaration -> TypeWithIdentifier "(" Args ")",
        BasicDeclaration -> TypeWithIdentifier "(" ")",
        FunctionDeclaration -> BasicDeclaration Block,
        ForwardDeclaration -> BasicDeclaration ";",

        Args -> TypeWithMaybeIdentifier,
        Args -> TypeWithMaybeIdentifier "," Args,

        StructOrUnionDeclaration -> StructOrUnion MaybeIdentifier "{" Members "}",
        StructOrUnion -> "struct",
        StructOrUnion -> "union",

        MaybeIdentifier -> ε,
        MaybeIdentifier -> Identifier,

        StructMembers -> ε,
        StructMembers -> StructMember StructMembers,
        StructMember -> TypeWithIdentifier MaybeBitfield ";",

        MaybeBitfield -> ":" NumericLiteral,
        MaybeBitfield -> ε,


        Block -> "{" BlockContents "}",
        BlockContents -> ε,
        BlockContents -> Statement BlockContents,

        Statement -> ";",
        Statement -> Declaration ";",
        Statement -> Expression ";",
        Statement -> ForLoop,
        Statement -> WhileLoop,
        Statement -> DoWhileLoop,
        Statement -> If,
        Statement -> Block,
        Statement -> Return,

        ExpressionOrDeclaration -> Expression,
        ExpressionOrDeclaration -> Declaration,
        ExpressionOrDeclaration -> ε,

        Declaration -> TypeWithMaybeIdentifier,
        Declaration -> TypeWithIdentifier "=" Expression,

        ForLoop -> "for" "(" ExpressionOrDeclaration ";" Expression ";" Expression ")" Statement,

        WhileLoop -> "while" "(" Expression ")" Statement,

        DoWhileLoop -> "do" Statement "while" "(" Expression ")" ";",

        If -> "if" "(" Expression ")" Statement MaybeElse,

        MaybeElse -> ε,
        MaybeElse -> "else" Statement,

        Return -> "return" Expression ";",

        Expression -> Expression15,
        ExpressionWithoutComma -> Expression14,

        Expression15 -> Expression14,
        Expression15 -> Expression15 "," Expression14,

        Expression14 -> Expression13,
        Expression14 -> Expression13 Operator14 Expression14,
        Operator14 -> "=",
        Operator14 -> "+=",
        Operator14 -> "-=",
        Operator14 -> "*=",
        Operator14 -> "/=",
        Operator14 -> "%=",
        Operator14 -> "<<=",
        Operator14 -> ">>=",
        Operator14 -> "&=",
        Operator14 -> "|=",
        Operator14 -> "^=",

        Expression13 -> Expression12,
        Expression13 -> Expression12 "?" Expression13 ":" Expression13,

        Expression12 -> Expression11,
        Expression12 -> Expression12 "||" Expression11,

        Expression11 -> Expression10,
        Expression11 -> Expression11 "&&" Expression10,

        Expression10 -> Expression9,
        Expression10 -> Expression10 "|" Expression9,

        Expression9 -> Expression8,
        Expression9 -> Expression9 "^" Expression8,

        Expression8 -> Expression7,
        Expression8 -> Expression8 "&" Expression7,

        Expression7 -> Expression6,
        Expression7 -> Expression7 Operator7 Expression6,
        Operator7 -> "==",
        Operator7 -> "!=",

        Expression6 -> Expression5,
        Expression6 -> Expression6 Operator6 Expression5,
        Operator6 -> ">=",
        Operator6 -> "<=",
        Operator6 -> ">",
        Operator6 -> "<",

        Expression5 -> Expression4,
        Expression5 -> Expression5 Operator5 Expression4,
        Operator5 -> "<<",
        Operator5 -> ">>",

        Expression4 -> Expression3,
        Expression4 -> Expression4 Operator4 Expression3,
        Operator4 -> "+",
        Operator4 -> "-",

        Expression3 -> Expression2,
        Expression3 -> Expression3 Operator3 Expression2,
        Operator3 -> "*",
        Operator3 -> "/",
        Operator3 -> "%",

        Expression2_5 -> Expression2,
        Expression2_5 -> "(" Type ")" Expression2_5,

        Expression2 -> Expression1,
        Expression2 -> Operator2 Expression2_5,
        Expression2 -> "sizeof" Expression2,
        Operator2 -> "--",
        Operator2 -> "++",
        Operator2 -> "+",
        Operator2 -> "-",
        Operator2 -> "!",
        Operator2 -> "~",
        Operator2 -> "*",
        Operator2 -> "&",

        Expression1 -> Identifier,
        Expression1 -> Literal,
        Expression1 -> Expression1 UnaryOperator1,
        Expression1 -> "(" Expression ")",
        Expression1 -> Expression1 "[" Expression "]",
        Expression1 -> Expression1 "(" CallArguments ")",
        UnaryOperator1 -> "++",
        UnaryOperator1 -> "--",
        UnaryOperator1 -> StructOperator Identifier,
        StructOperator -> ".",
        StructOperator -> "->",
        CallArguments -> ε,
        CallArguments -> NonEmptyCallArguments,
        NonEmptyCallArguments -> ExpressionWithoutComma "," NonEmptyCallArguments,
        NonEmptyCallArguments -> ExpressionWithoutComma,

    //TODO: Compound literals (who uses those...),

        Literal -> NumericLiteral,
        Literal -> StringLiteral,
    )
}
