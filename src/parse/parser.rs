#![allow(unused_variables)]
use super::parsetreetypes::{ParseNode, ParseNodeType};
use super::rules::get_rules;
use crate::lex::constants::LexItem;
use crate::lex::errors::{LexResult, LexSuccess};
use std::cell::RefCell;
use std::mem::discriminant;
use std::rc::Rc;

#[derive(Clone)]
struct RuleState<'a> {
    pub result: ParseNodeType,
    pub rule: &'a [ParseNodeType],
    pub position: usize,
    pub parent: Option<Rc<RuleState<'a>>>,
    pub self_node: Rc<RefCell<ParseNode>>,
}

impl<'a> RuleState<'a> {
    pub fn new_start(rules: &[(ParseNodeType, Vec<ParseNodeType>)]) -> RuleState {
        let rule = *search_rule(rules, &ParseNodeType::Start).first().unwrap();
        RuleState {
            result: ParseNodeType::Start,
            rule,
            position: 0,
            parent: None,
            self_node: Rc::new(RefCell::new(ParseNode {
                node_type: ParseNodeType::Start,
                children: vec![],
            })),
        }
    }
}

struct ParserState<'a, T: Iterator<Item = LexResult>> {
    it: T,
    lookahead: Vec<LexResult>,
    rule: RuleState<'a>,
}

fn search_rule<'a>(
    rules: &'a [(ParseNodeType, Vec<ParseNodeType>)],
    request: &ParseNodeType,
) -> Vec<&'a Vec<ParseNodeType>> {
    rules
        .iter()
        .filter(|rule| rule.0 == *request)
        .map(|rule| &rule.1)
        .collect()
}

fn next_tok<T: Iterator<Item = LexResult>>(state: &mut ParserState<T>) -> Option<LexResult> {
    state.lookahead.pop().or_else(|| state.it.next())
}

fn next_success<T: Iterator<Item = LexResult>>(state: &mut ParserState<T>) -> Option<LexSuccess> {
    next_tok(state)?.ok()
}

pub fn parse<T: Iterator<Item = LexResult>>(tokens: T) -> Rc<RefCell<ParseNode>> {
    let rules = get_rules();
    let mut state = ParserState {
        it: tokens,
        lookahead: Vec::new(),
        rule: RuleState::new_start(&rules),
    };

    'outer: loop {
        while state.rule.position == state.rule.rule.len() {
            state.rule = ((if let Some(parent) = state.rule.parent {
                parent
            } else {
                break 'outer;
            })
            .as_ref())
            .clone();
            state.rule.position += 1;
        }

        let needed = &state.rule.rule[state.rule.position];

        if let ParseNodeType::Lex(ref lex_req) = needed {
            let tok = if let Some(tok) = next_success(&mut state) {
                tok
            } else {
                break 'outer;
            };
            if discriminant(&tok.item) == *lex_req {
                let mut node = state.rule.self_node.borrow_mut();
                node.children
                    .push(Rc::new(RefCell::new(ParseNode::from_lex(tok))));
                continue;
            } else {
                panic!(
                    "Unexpected {:?} at line {} column {}",
                    tok.item, tok.line, tok.column
                );
            }
        } else if let ParseNodeType::Keyword(ref kw_req) = needed {
            let tok = if let Some(tok) = next_success(&mut state) {
                tok
            } else {
                break 'outer;
            };

            if let LexItem::Keyword(ref tok_kw) = tok.item {
                if discriminant(tok_kw) == *kw_req {
                    let mut node = state.rule.self_node.borrow_mut();
                    node.children
                        .push(Rc::new(RefCell::new(ParseNode::from_lex(tok))));
                    continue;
                } else {
                    panic!(
                        "Unexpected {:?} at line {} column {}",
                        &tok.item, tok.line, tok.column
                    );
                }
            } else {
                panic!(
                    "Expected keyword {:?}, found {:?} at line {} column {}",
                    kw_req, tok.item, tok.line, tok.column
                )
            }
        }

        let new_rules = search_rule(&rules, needed);
        if new_rules.len() > 1 {
            unimplemented!()
        } else if let Some(rule) = new_rules.first() {
            state.rule = RuleState {
                result: needed.clone(),
                rule: &rule,
                position: 0,
                parent: Some(Rc::new(state.rule)),
                self_node: Rc::new(RefCell::new(ParseNode {
                    node_type: needed.clone(),
                    children: vec![],
                })),
            };
            if let Some(ref parent_rule) = state.rule.parent {
                let mut parent_node = parent_rule.self_node.borrow_mut();
                parent_node.children.push(state.rule.self_node.clone());
            }
        } else {
            panic!(format!("No such rule for {:?}", *needed))
        }
    }

    if state.rule.parent.is_none() {
        state.rule.self_node
    } else {
        panic!("Did not finish parsing, more tokens needed")
    }
}
