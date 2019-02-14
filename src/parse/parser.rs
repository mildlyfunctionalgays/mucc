#![allow(unused_variables)]
use super::types::{ParseNode, ParseNodeType};
use crate::lex::errors::{LexResult, LexSuccess};
use crate::lex::types::LexItem;
use crate::parse::types::NonTerminalType;
use crate::parse::types::RuleType;
use std::mem::discriminant;
use std::mem::Discriminant;
use std::rc::Rc;

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
        mut self,
        rules: &[(NonTerminalType, &'a [RuleType])],
    ) -> impl IntoIterator<Item = RuleState<'a>> {
        while self.self_node.children.len() == self.rule.len() {
            let RuleState {
                rule,
                parent,
                self_node,
            } = self;

            if let Some(parent) = parent {
                self = Rc::unwrap_or_clone(parent);
                let new_node = Rc::make_mut(&mut self.self_node);
                new_node.children.push(self_node);
            } else {
                self = RuleState {
                    rule,
                    parent: None,
                    self_node,
                };
                return vec![self];
            }
        }
        let next_rule = &self.rule[self.self_node.children.len()];
        match next_rule {
            RuleType::Terminal(item) => vec![self],
            RuleType::NonTerminal(needed) => {
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
        println!("{:#?}", states);
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
