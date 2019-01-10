#![allow(unused_variables, unused_imports)]
use super::parsetreetypes::{ParseNode, ParseNodeType};
use super::rules::RULES;
use crate::lex::errors::{LexError, LexResult};
use crate::lex::constants::{LexItem, LexKeyword};
use std::rc::Weak;
use std::mem::discriminant;
use std::rc::Rc;

struct RuleState {
    pub result: ParseNodeType,
    pub rule: &'static [ParseNodeType],
    pub position: usize,
    pub self_node: Box<ParseNode>,
    pub current_child: Option<Box<RuleState>>,
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
            current_child: None,
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
                let mut parent = rule;

                'outer: loop {
                    let mut child = parent.current_child;
                    if let Some(ref mut child) = parent.current_child {
                        let mut child = child;
                        while child.position >= child.rule.len() - 1 {
                            if let Some(ref mut grandchild) = child.current_child {
                                child = grandchild;
                            } else {
                                break 'outer;
                            }
                        }
                        parent = child;
                    } else {
                        break
                    }
                }
                loop {
                    let mut parent2 = parent;
                    while let Some(ref mut child) = parent2.current_child {
                        if child.current_child.is_some() {
                            parent2 = child;
                        } else {
                            break;
                        }
                    }
                    let delete = if let Some(child) = parent2.current_child {
                        child.position == child.rule.len()
                    } else {
                        false
                    };
                    if delete {
                        parent2.position += 1;
                        parent2.current_child = None;
                    }
                }

                let needed = child.rule[child.position];
                if let ParseNodeType::Keyword(kw) = needed {
                    if let LexItem::Keyword(kw_tok) = tok {
                        if discriminant(&kw_tok) == kw {
                            unimplemented!()
                        } else {
                            to_delete.push(idx);
                            break;
                        }
                    } else {
                        to_delete.push(idx);
                        break;
                    }
                } else if let ParseNodeType::Lex(item) = needed {
                    if discriminant(&tok) == item {
                        unimplemented!()
                    } else {
                        to_delete.push(idx);
                        break;
                    }
                } else {
                    let rules = search_rule(&needed)
                        .iter()
                        .map(|rule| {
                            RuleState {
                                result: needed.clone(),
                                rule: *rule,
                                position: 0,
                                self_node: Box::new(ParseNode {
                                    node_type: needed.clone(),
                                    children: vec![],
                                    line: 0,
                                    column: 0
                                }),
                                current_child: None
                            }
                        });
                }

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
