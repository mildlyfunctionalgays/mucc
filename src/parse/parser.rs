#![allow(unused_variables, unused_imports)]
use super::parsetreetypes::{ParseNode, ParseNodeType};
use super::rules::RULES;
use crate::lex::errors::{LexError, LexResult};

struct RuleState {
    pub result: ParseNodeType,
    pub rule: &'static [ParseNodeType],
    pub position: usize,
    pub self_node: Box<ParseNode>,
    pub current_child: Box<Option<RuleState>>,
}

impl RuleState {
    pub fn new_start(rule: &'static [ParseNodeType]) -> RuleState {
        RuleState {
            result: ParseNodeType::Start,
            rule,
            position: 0,
            self_node: Box::new(ParseNode {
                node_type: ParseNodeType::Start,
                children: vec![],
                line: 1,
                column: 1,
            }),
            current_child: Box::new(None),
        }
    }
}

fn search_rule(request: &ParseNodeType) -> Vec<&'static [ParseNodeType]> {
    RULES
        .iter()
        .filter(|rule| rule.0 == *request)
        .map(|rule| rule.1)
        .collect()
}

pub fn parse<T: Iterator<Item = LexResult>>(tokens: T) -> Box<ParseNode> {
    let mut candidate_rules = search_rule(&ParseNodeType::Start)
        .iter()
        .map(|rule| RuleState::new_start(*rule))
        .collect::<Vec<_>>();

    for item in tokens {
        if let Ok(tok) = item.item {
            let mut to_delete = Vec::new();
            for (idx, rule) in candidate_rules.iter_mut().enumerate() {
                to_delete.push(idx);
            }
            to_delete.iter().rev().for_each(|idx| {
                candidate_rules.remove(*idx);
            });
        } else {
            panic!("Error in lexer");
        }
    }

    if candidate_rules.len() > 1 {
        panic!(format!(
            "Grammar is ambiguous, found {} possible matches",
            candidate_rules.len()
        ))
    } else if let Some(rule) = candidate_rules.first() {
        rule.self_node.clone()
    } else {
        panic!("No matches found")
    }
}
