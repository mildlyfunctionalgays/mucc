macro_rules! require_non_terminal {
    ($node:expr, $typ:expr) => {
        if ($node).node_type != ParseNodeType::NonTerminal($typ) {
            panic!(
                "Attempted to treat {:?} node as {:?} while building untyped AST",
                ($node).node_type,
                $typ
            );
        }
    };
}

macro_rules! require_len {
    ($node:expr, $rule:expr) => {
        if !($rule($node.children.len())) {
            panic!(
                "Found {:?} with invalid length {} while building untyped AST",
                ($node).node_type,
                ($node).children.len()
            );
        }
    };
}
