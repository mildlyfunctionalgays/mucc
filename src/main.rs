mod lex;
use crate::lex::lexer::Lexer;
use std::fs::File;
use std::env;
use std::io::Read;

/// A super simple main function which lexes
fn main() -> std::io::Result<()> {
    if env::args().len() != 2 {
        eprintln!("Usage: µcc <filename>");
        return Ok(());
    }
    let filename = env::args().nth(1).unwrap();
    let mut file = File::open(filename).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut chars = code.chars();
    let tokens = Lexer::new(&mut chars);

    for token in tokens {
        println!("Got token {:?}", token);
    }

    return Ok(());
}
