#![recursion_limit = "1000"]
//#![feature(trace_macros)]
#![allow(dead_code)]
mod lex;
mod parse;
use crate::lex::lexer::Lexer;
use crate::parse::parser::parse;
use std::env;
use std::fs::File;
use std::io::Read;
#[cfg(fuzzing)]
use afl::fuzz;

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

    let tree = parse(tokens);
    println!("Got tree {:?}", tree);

    Ok(())
}

#[cfg(fuzzing)]
fn main() {
    fuzz!(|data| {
        if let Ok(text) = std::str::from_utf8(data) {
            Lexer::new(text.chars()).for_each(drop);
        }
    })
}