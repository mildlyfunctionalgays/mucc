#![recursion_limit = "1000"]
mod cpp;
mod lex;
mod parse;
mod untyped_ast;

use crate::lex::Lexer;
use crate::parse::parser::parse;
#[cfg(not(fuzzing))]
use std::env;
#[cfg(not(fuzzing))]
use std::fs::File;
#[cfg(not(fuzzing))]
use std::io::Read;

use crate::untyped_ast::build_untyped_ast;
#[cfg(fuzzing)]
//use afl::fuzz;
use honggfuzz::fuzz;

/// A super simple main function which lexes
#[cfg(not(fuzzing))]
fn main() -> std::io::Result<()> {
    if env::args().len() != 2 {
        eprintln!("Usage: μcc <filename>");
        return Ok(());
    }
    let filename = env::args().nth(1).unwrap();
    let mut file = File::open(filename).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let chars = code.chars();
    let tokens = Lexer::new(chars);

    let tree = parse(tokens).unwrap();

    let u_ast = build_untyped_ast(tree);
    println!("Got tree {:?}", u_ast);

    Ok(())
}

#[cfg(fuzzing)]
fn main() {
    loop {
        fuzz!(|data| {
            // Comment to prevent rustfmt from BREAKING this code
            if let Ok(text) = std::str::from_utf8(data) {
                let _ = Lexer::new(text.chars()).for_each(drop);
            }
        });
    }
}
