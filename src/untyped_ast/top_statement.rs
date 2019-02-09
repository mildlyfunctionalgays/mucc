use crate::lex::types::LexItem;
use crate::parse::types::NonTerminalType;
use crate::parse::types::ParseNode;
use crate::parse::types::ParseNodeType;
use crate::untyped_ast::types::TopStatement;
use crate::untyped_ast::types::Type;
use std::rc::Rc;

pub(super) fn read_top_statements(node: Rc<ParseNode>) -> Vec<TopStatement> {
    require_non_terminal!(node, NonTerminalType::TopStatements);

    let mut statements = Vec::new();

    for child in node.children.clone() {
        match child.node_type {
            ParseNodeType::NonTerminal(NonTerminalType::TopStatement) => {
                statements.push(read_top_statement(child))
            }
            ParseNodeType::NonTerminal(NonTerminalType::TopStatements) => {
                statements.extend(read_top_statements(child).into_iter())
            }
            _ => unreachable!(),
        }
    }

    statements
}

pub(super) fn read_top_statement(node: Rc<ParseNode>) -> TopStatement {
    require_non_terminal!(node, NonTerminalType::TopStatement);
    require_len!(node, |len| len == 1);

    let child = &node.children[0];

    match child.node_type {
        ParseNodeType::NonTerminal(NonTerminalType::Declaration) => unimplemented!(),
        ParseNodeType::NonTerminal(NonTerminalType::ForwardDeclaration) => {
            read_forward_declaration(child.clone())
        }
        ParseNodeType::NonTerminal(NonTerminalType::FunctionDeclaration) => {
            read_function_declaration(child.clone())
        }
        ParseNodeType::NonTerminal(NonTerminalType::StructOrUnionDeclaration) => unimplemented!(),
        ParseNodeType::NonTerminal(NonTerminalType::Typedef) => unimplemented!(),
        _ => unreachable!(),
    }
}

fn read_forward_declaration(node: Rc<ParseNode>) -> TopStatement {
    require_non_terminal!(node, NonTerminalType::ForwardDeclaration);
    require_len!(node, |len| len == 2);
    require_terminal!(node, 1, LexItem::Semicolon);

    let (ret_type, name, args) = read_basic_declaration(node.children[0].clone());
    let args_typ = args.iter().map(|item| item.0.clone()).collect();
    TopStatement::ForwardDeclaration(ret_type, name, args_typ)
}

fn read_function_declaration(node: Rc<ParseNode>) -> TopStatement {
    require_non_terminal!(node, NonTerminalType::FunctionDeclaration);
    require_len!(node, |len| len == 2);

    unimplemented!()
}

fn read_basic_declaration(node: Rc<ParseNode>) -> (Type, String, Vec<(Type, Option<String>)>) {
    require_non_terminal!(node, NonTerminalType::BasicDeclaration);
    require_len!(node, |len| len == 4 || len == 3);
    require_terminal!(node, 1, LexItem::LeftParen);

    if node.children.len() == 3 {
        require_terminal!(node, 2, LexItem::RightParen);
        let (typ, name) = read_type_with_identifier(node.children[0].clone());
        let args = Vec::new();
        (typ, name, args)
    } else {
        require_terminal!(node, 3, LexItem::RightParen);
        unimplemented!()
    }
}

fn read_type_with_identifier(node: Rc<ParseNode>) -> (Type, String) {
    require_non_terminal!(node, NonTerminalType::TypeWithIdentifier);
    require_len!(node, |len| len == 2);

    //TODO: Function pointer support

    let typ = read_type(node.children[0].clone());
    let ident = read_identifier(node.children[1].clone());

    (typ, ident)
}

fn read_type(node: Rc<ParseNode>) -> Type {
    require_non_terminal!(node, NonTerminalType::Type);

    // TODO: Make this work, currently it's just for testing
    Type::SignedInt
}

fn read_identifier(node: Rc<ParseNode>) -> String {
    match node.node_type.clone() {
        ParseNodeType::Terminal(succ) => match succ.item {
            LexItem::Identifier(s) => s,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
