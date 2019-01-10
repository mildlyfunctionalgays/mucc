use crate::parse::parsetreetypes::ParseNodeType::{self, *};
/*
Start -> ε,
Start -> TopStatement Start,

TopStatement -> Type MaybeIdentifier ";",
TopStatement -> Typedef,
TopStatement -> FunctionDeclaration,
TopStatement -> ForwardDeclaration,
TopStatement -> ";",

Typedef -> "typedef" MaybeType Identifier ";",
FunctionPointer -> TypeWithoutIdentifier "(",
Type -> BasicType,
Type -> Type "*",
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
TypeSpecifier -> StructOrUnionDeclaration,
MaybeType -> ε,
MaybeType -> Type,

BasicDeclaration -> Type Identifier "(" Args ")",
FunctionDeclaration -> BasicDeclaration Block,
ForwardDeclaration -> BasicDeclaration ";",


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
Statement -> ForLoop,
Statement -> WhileLoop,
Statement -> DoWhileLoop,
Statement -> If,
Statement -> Block,

Declaration -> Type Identifier,
Declaration -> Type,
Declaration -> Type Identifier "=" Expression,

ExpressionOrDeclaration -> Expression,
ExpressionOrDeclaration -> Declaration,
ExpressionOrDeclaration -> ε,

ForLoop -> "for" "(" ExpressionOrDeclaration ";" Expression ";" Expression ")" Statement,

WhileLoop -> "while" "(" Expression ")" Statement,

DoWhileLoop -> "do" Statement "while" "(" Expression ")" ";",

If -> "if" "(" Expression ")" Statement,
If -> "if" "(" Expression ")" Statement "else" Statement,

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
Literal -> FloatLiteral,

*/

pub const RULES: &[(ParseNodeType, &[ParseNodeType])] =
    &[(Start, &[]), (Start, &[TopStatement, Start])];
