#![allow(unused_variables)]
use super::parsetreetypes::{ParseNode, ParseNodeType};
use super::rules::RULES;
use crate::lex::errors::{LexResult, LexSuccess};
use std::rc::Rc;

#[derive(Clone)]
struct RuleState {
    pub result: ParseNodeType,
    pub rule: &'static [ParseNodeType],
    pub position: usize,
    pub parent: Option<Rc<RuleState>>,
    pub self_node: Rc<ParseNode>,
}

impl RuleState {
    pub fn new_start() -> RuleState {
        let rule = *search_rule(&ParseNodeType::Start).first().unwrap();
        RuleState {
            result: ParseNodeType::Start,
            rule,
            position: 0,
            parent: None,
            self_node: Rc::new(ParseNode {
                node_type: ParseNodeType::Start,
                children: vec![],
            }),
        }
    }
}

struct ParserState<T: Iterator<Item = LexResult>> {
    it: T,
    lookahead: Vec<LexResult>,
    rule: RuleState,
}

fn search_rule(request: &ParseNodeType) -> Vec<&'static [ParseNodeType]> {
    RULES
        .iter()
        .filter(|rule| rule.0 == *request)
        .map(|rule| rule.1)
        .collect()
}

fn next_tok<T: Iterator<Item = LexResult>>(state: &mut ParserState<T>) -> Option<LexResult> {
    state.lookahead.pop().or_else(|| state.it.next())
}

fn next_success<T: Iterator<Item = LexResult>>(state: &mut ParserState<T>) -> Option<LexSuccess> {
    next_tok(state)?.ok()
}

pub fn parse<T: Iterator<Item = LexResult>>(tokens: T) -> Rc<ParseNode> {
    let mut state = ParserState {
        it: tokens,
        lookahead: Vec::new(),
        rule: RuleState::new_start(),
    };

    while let Some(tok) = next_success(&mut state) {
        while state.rule.position == state.rule.rule.len() {
            state.rule = (*state.rule.parent.unwrap().as_ref()).clone();
            state.rule.position += 1;
        }

        let needed = &state.rule.rule[state.rule.position];

        let new_rules = search_rule(needed);
        if new_rules.len() > 1 {
            unimplemented!()
        } else if let Some(rule) = new_rules.first() {
            unimplemented!()
        } else {
            panic!(format!("No such rule for {:?}", *needed))
        }
    }

    if state.rule.parent.is_none() {
        state.rule.self_node.clone()
    } else {
        panic!("Did not finish parsing, more tokens needed")
    }
}
