#![allow(unused_variables)]
use super::types::{ParseNode, ParseNodeType};
use crate::lex::errors::{LexResult, LexSuccess};
use crate::lex::types::LexItem;
use crate::parse::types::NonTerminalType;
use crate::parse::types::RuleType;
use std::mem::discriminant;
use std::mem::Discriminant;
use std::rc::Rc;

use lazy_static::lazy_static;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter;

#[derive(Clone, Debug)]
struct RuleState<'a> {
    pub rule: &'a [RuleType],
    pub parent: Option<Rc<RuleState<'a>>>,
    pub self_node: Rc<ParseNode>,
}

trait UnwrapOrClone {
    type Item;
    fn unwrap_or_clone(self) -> Self::Item;
}

impl<T: Clone> UnwrapOrClone for Rc<T> {
    type Item = T;

    fn unwrap_or_clone(self) -> T {
        Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone())
    }
}

fn find_non_terminal<'a>(
    non_terminal: NonTerminalType,
    intermediate: Vec<(NonTerminalType, &'a [RuleType])>,
    rules: &[(NonTerminalType, &'a [RuleType])],
) -> Vec<Vec<(NonTerminalType, &'a [RuleType])>> {
    let last = *intermediate.last().unwrap();
    match last.1.get(0) {
        Some(RuleType::NonTerminal(current_rule)) => {
            if *current_rule == non_terminal {
                return vec![intermediate];
            }
            for (_, int) in intermediate.iter().rev().skip(1) {
                if int[0] == RuleType::NonTerminal(*current_rule) {
                    return Vec::new();
                }
            }
            rules
                .into_iter()
                .filter(|(key, _)| *key == *current_rule)
                .flat_map(|(key, value)| {
                    let mut clone = intermediate.clone();
                    clone.push((*key, value));
                    find_non_terminal(non_terminal, clone, rules).into_iter()
                })
                .collect()
        }

        _ => Vec::new(),
    }
}

fn left_recursions_map(
) -> HashMap<NonTerminalType, Vec<Vec<(NonTerminalType, &'static [RuleType])>>> {
    let mut map: HashMap<NonTerminalType, Vec<Vec<(NonTerminalType, &'static [RuleType])>>> = HashMap::new();
    let rules = *super::rules::RULES;

    for rule in rules {
        let key = rule.0;
        match map.entry(key) {
            Entry::Occupied(mut o) => {
                o.get_mut().extend_from_slice(&find_non_terminal(key, vec![*rule], rules));
            },
            Entry::Vacant(v) => {
                v.insert(find_non_terminal(key, vec![*rule], rules));
            },
        };
    }
    println!("map = {:#?}", map);
    map
}

lazy_static! {
    static ref LEFT_RECURSIONS: HashMap<NonTerminalType, Vec<Vec<(NonTerminalType, &'static [RuleType])>>> =
        left_recursions_map();
}

impl<'a> RuleState<'a> {
    pub fn new_start(rules: &[(NonTerminalType, &'a [RuleType])]) -> RuleState<'a> {
        let rule = {
            let rules = search_rule(rules, &NonTerminalType::Start);
            if rules.len() > 2 {
                panic!("There is more than one Start rule, which is not allowed.")
            } else {
                *rules.first().unwrap()
            }
        };
        RuleState {
            rule,
            parent: None,
            self_node: Rc::new(ParseNode {
                node_type: ParseNodeType::NonTerminal(NonTerminalType::Start),
                children: Vec::new(),
            }),
        }
    }
    fn match_token(
        self,
        token: &LexSuccess,
        rules: &[(NonTerminalType, &'a [RuleType])],
    ) -> Result<RuleState<'a>, Option<Discriminant<LexItem>>> {
        let index = self.self_node.children.len();
        if self.rule.len() == index {
            return Err(None);
        }
        let next_rule = &self.rule[index];
        if let RuleType::Terminal(item) = next_rule {
            if discriminant(&token.item) == *item {
                let mut state = self;
                let node = Rc::make_mut(&mut state.self_node);
                node.children.push(Rc::new(ParseNode {
                    node_type: ParseNodeType::Terminal(token.clone()),
                    children: Vec::new(),
                }));
                Ok(state)
            } else {
                Err(Some(*item))
            }
        } else {
            unreachable!();
        }
    }
    fn move_forward(
        self,
        rules: &[(NonTerminalType, &'a [RuleType])],
    ) -> impl IntoIterator<Item = RuleState<'a>> {
        if let Some(next_rule) = self.rule.get(self.self_node.children.len()) {
            match next_rule {
                RuleType::Terminal(item) => vec![self],
                RuleType::NonTerminal(needed) if !self.looped_token(*needed) => {
                    let matched_rules = search_rule(rules, needed);

                    let self_rc = Rc::new(self);

                    matched_rules
                        .into_iter()
                        .map(|rule| RuleState {
                            rule,
                            parent: Some(self_rc.clone()),
                            self_node: Rc::new(ParseNode {
                                node_type: ParseNodeType::NonTerminal(needed.clone()),
                                children: Vec::new(),
                            }),
                        })
                        .map(|rule_state| rule_state.move_forward(rules))
                        .flatten()
                        .collect()
                }
                _ => Vec::new(),
            }
        } else {
            let RuleState {
                rule,
                parent,
                self_node,
            } = self;

            if let Some(parent) = parent {
                let non_terminal = match parent.rule[parent.self_node.children.len()] {
                    RuleType::Terminal(_) => unreachable!(),
                    RuleType::NonTerminal(ref non_terminal) => non_terminal,
                };
                let recursions = LEFT_RECURSIONS.get(non_terminal).unwrap();
                recursions.iter().map(|recursion| {
                    recursion.iter().fold((*parent).clone(), |node, &(non_terminal, rule): &(NonTerminalType, &[RuleType])| {
                        RuleState {
                            rule,
                            parent: Some(Rc::new(node)),
                            self_node: Rc::new(ParseNode {
                                node_type: ParseNodeType::NonTerminal(non_terminal),
                                children: vec![]
                            }),
                        }
                    })
                }).chain(iter::once((*parent).clone())).flat_map(|mut node| {
                    let new_node = Rc::make_mut(&mut node.self_node);
                    new_node.children.push(self_node.clone());
                    node.move_forward(rules).into_iter()
                }).collect()
            } else {
                vec![RuleState {
                    rule,
                    parent: None,
                    self_node,
                }]
            }
        }
    }
    fn looped_token(&self, token: NonTerminalType) -> bool {
        if self.self_node.children.len() > 0 {
            false
        } else if let Some(ref parent) = self.parent {
            parent.rule[parent.self_node.children.len()] == RuleType::NonTerminal(token)
                || parent.as_ref().looped_token(token)
        } else {
            false
        }
    }
}

fn search_rule<'a>(
    rules: &[(NonTerminalType, &'a [RuleType])],
    request: &NonTerminalType,
) -> Vec<&'a [RuleType]> {
    rules
        .iter()
        .filter(|rule| rule.0 == *request)
        .map(|rule| rule.1)
        .collect()
}

pub fn parse<T: Iterator<Item = LexResult>>(
    mut tokens: T,
) -> Result<Rc<ParseNode>, Vec<Option<Discriminant<LexItem>>>> {
    let rules = &*super::rules::RULES;

    let mut states: Vec<RuleState> = vec![RuleState::new_start(rules)];

    loop {
        print!("{} ", states.len());
        states = states
            .into_iter()
            .flat_map(|state| state.move_forward(rules))
            .collect();

        let token = {
            if let Some(token) = tokens.next() {
                token.ok().unwrap()
            } else {
                break;
            }
        };

        let state_results: (Vec<_>, Vec<_>) = states
            .into_iter()
            .map(|state| state.match_token(&token, rules))
            .partition(Result::is_ok);
        states = state_results.0.into_iter().map(Result::unwrap).collect();

        if states.is_empty() {
            let expected: Vec<Option<Discriminant<LexItem>>> = state_results
                .1
                .into_iter()
                .map(Result::unwrap_err)
                .collect();
            dbg!(&expected);
            return Err(expected);
        }
    }

    states = states
        .into_iter()
        .flat_map(|state| state.move_forward(rules))
        .collect();

    states = states
        .into_iter()
        .filter(|state| state.self_node.children.len() == state.rule.len())
        .collect();

    if states.len() > 1 {
        dbg!(states);
        unimplemented!()
    } else if let Some(state) = states.into_iter().next() {
        if state.parent.is_some() {
            unimplemented!()
        }
        Ok(state.self_node)
    } else {
        unimplemented!()
    }
}
