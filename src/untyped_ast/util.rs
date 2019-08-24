macro_rules! require_non_terminal {
    ($node:expr, $typ:expr) => {
        debug_assert_eq!(
            ($node).node_type,
            ParseNodeType::NonTerminal($typ),
            "Attempted to treat {:?} node as {:?} while building untyped AST",
            ($node).node_type,
            $typ
        );
    };
}

macro_rules! require_terminal {
    ($node:expr, $idx:expr, $typ:expr) => {
        match &($node).children[($idx)].clone().node_type {
            ParseNodeType::Terminal(s) => {
                debug_assert_eq!(
                    s.item,
                    ($typ),
                    "Node of type {:?} requires a {:?} at index {}, found {:?}",
                    ($node),
                    ($typ),
                    ($idx),
                    s.item
                );
            }
            _ => {
                debug_assert!(
                    false,
                    "Node of type {:?} requires a terminal token at index {}",
                    ($node).node_type,
                    ($idx)
                );
            }
        }
    };
}

macro_rules! require_len {
    ($node:expr, $rule:expr) => {
        debug_assert!(
            $rule($node.children.len()),
            "Found {:?} with invalid length {} while building untyped AST",
            ($node).node_type,
            ($node).children.len()
        );
    };
}

macro_rules! non_terminal_rule {
    (-> $func:expr; $node:expr) => {
        vec![$func($node)]
    };
    (=> $func:expr; $node:expr) => {
        $func($node)
    };
}

macro_rules! collapse_non_terminal {
    ($node:expr, $($rule:ident $tok:tt $func:expr),*) => {
        $node.children.iter().flat_map(|child| {
               match child.node_type {
                    $(
                        crate::parse::types::ParseNodeType::NonTerminal(NonTerminalType::$rule) => non_terminal_rule!($tok $func; child.clone()),
                    )*
                    _ => unreachable!()
               }
        }).collect()
    }
}
