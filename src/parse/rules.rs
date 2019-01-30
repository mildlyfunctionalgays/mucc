use crate::lex::constants::{LexItem, NumberType};
use crate::parse::parsetreetypes::ParseNodeType::{self, *};
use lazy_static::lazy_static;
use std::mem::discriminant;

macro_rules! grammar {
    (@token StringLiteral) => {
        ParseNodeType::Lex(discriminant(&LexItem::StringLiteral(Vec::new())))
    };
    (@token StringLiteral) => {
        ParseNodeType::Lex(discriminant(&LexItem::StringLiteral(Vec::new())))
    };
    (@token Identifier) => {
        ParseNodeType::Lex(discriminant(&LexItem::Identifier(String::new())))
    };
    (@token NumericLiteral) => {
        ParseNodeType::Lex(discriminant(&LexItem::NumericLiteral(NumberType::default())))
    };
    (@token $token:ident) => {
        $token
    };
    (@token $token:tt) => {
        {
            ParseNodeType::from($token)
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
    pub static ref RULES: &'static [(ParseNodeType, &'static [ParseNodeType])] = &*RULE_VEC_2;
    static ref RULE_VEC_1: Vec<(ParseNodeType, Vec<ParseNodeType>)> = get_rules();
    static ref RULE_VEC_2: Vec<(ParseNodeType, &'static [ParseNodeType])> = {
        (*RULE_VEC_1)
            .iter()
            .map(|&(ref key, ref value)| (key.clone(), value.as_slice()))
            .collect()
    };
}

pub fn get_rules() -> Vec<(ParseNodeType, Vec<ParseNodeType>)> {
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
        Type -> BasicType Pointers,
        Pointers -> ε,
        Pointers -> "*" Pointers,

        BasicType -> TypeSpecifier,
        BasicType -> TypeSpecifier Type,
        TypeSpecifier -> "int",
        TypeSpecifier -> "long",
        TypeSpecifier -> "float",
        TypeSpecifier -> "short",
        TypeSpecifier -> "char",
        TypeSpecifier -> "volatile",
        TypeSpecifier -> "double",
        TypeSpecifier -> "register",
        TypeSpecifier -> "unsigned",
        TypeSpecifier -> "signed",
        TypeSpecifier -> StructOrUnionDeclaration,
        MaybeType -> ε,
        MaybeType -> Type,

        BasicDeclaration -> Type Identifier "(" Args ")",
        BasicDeclaration -> Type Identifier "(" ")",
        FunctionDeclaration -> BasicDeclaration Block,
        ForwardDeclaration -> BasicDeclaration ";",

        Args -> Type MaybeIdentifier,
        Args -> Type MaybeIdentifier "," Args,

        StructOrUnionDeclaration -> StructOrUnion MaybeIdentifier "{" Members "}",
        StructOrUnion -> "struct",
        StructOrUnion -> "union",

        MaybeIdentifier -> ε,
        MaybeIdentifier -> Identifier,

        StructMembers -> ε,
        StructMembers -> StructMember StructMembers,
        StructMember -> Type Identifier MaybeBitfield ";",

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

        Declaration -> Type MaybeIdentifier,
        Declaration -> Type Identifier "=" Expression,

        ForLoop -> "for" "(" ExpressionOrDeclaration ";" Expression ";" Expression ")" Statement,

        WhileLoop -> "while" "(" Expression ")" Statement,

        DoWhileLoop -> "do" Statement "while" "(" Expression ")" ";",

        If -> "if" "(" Expression ")" Statement,
        If -> "if" "(" Expression ")" Statement "else" Statement,

        Return -> "return" Expression ";",

        Expression -> Expression15,
        ExpressionWithoutComma -> Expression14,

        Expression15 -> Expression14 Expression15Trail,
        Expression15Trail -> ε,
        Expression15Trail -> "," Expression14 Expression15Trail,

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

        Expression12 -> Expression11 Expression12Trail,
        Expression12Trail -> ε,
        Expression12Trail -> "||" Expression11 Expression12Trail,

        Expression11 -> Expression10 Expression11Trail,
        Expression11Trail -> ε,
        Expression11Trail -> "&&" Expression10 Expression11Trail,

        Expression10 -> Expression9 Expression10Trail,
        Expression10Trail -> ε,
        Expression10Trail -> "|" Expression9 Expression10Trail,

        Expression9 -> Expression8 Expression9Trail,
        Expression9Trail -> ε,
        Expression9Trail -> "^" Expression8 Expression9Trail,

        Expression8 -> Expression7 Expression8Trail,
        Expression8Trail -> ε,
        Expression8Trail -> "&" Expression7 Expression8Trail,

        Expression7 -> Expression6 Expression7Trail,
        Expression7Trail -> ε,
        Expression7Trail -> Operator7 Expression6 Expression7Trail,
        Operator7 -> "==",
        Operator7 -> "!=",

        Expression6 -> Expression5 Expression6Trail,
        Expression6Trail -> ε,
        Expression6Trail -> Operator6 Expression5 Expression6Trail,
        Operator6 -> ">=",
        Operator6 -> "<=",
        Operator6 -> ">",
        Operator6 -> "<",

        Expression5 -> Expression4 Expression5Trail,
        Expression5Trail -> ε,
        Expression5Trail -> Operator5 Expression4 Expression5Trail,
        Operator5 -> "<<",
        Operator5 -> ">>",

        Expression4 -> Expression3 Expression4Trail,
        Expression4Trail -> ε,
        Expression4Trail -> Operator4 Expression3 Expression4Trail,
        Operator4 -> "+",
        Operator4 -> "-",

        Expression3 -> Expression2 Expression3Trail,
        Expression3Trail -> ε,
        Expression3Trail -> Operator3 Expression2 Expression3Trail,
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

        Expression1 -> Expression1NoTrail Expression1Trail,
        Expression1NoTrail -> Identifier,
        Expression1NoTrail -> Literal,
        Expression1NoTrail -> "(" Expression ")",
        Expression1Trail -> ε,
        Expression1Trail -> UnaryOperator1 Expression1Trail,
        Expression1Trail -> "[" Expression "]" Expression1Trail,
        Expression1Trail -> "(" CallArguments ")" Expression1Trail,
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
