use crate::lex::errors::LexResult;
use crate::lex::lexer::Lexer;
use crate::parse::parser::parse;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

#[test]
fn test_parse_empty() {
    let text = "\n";
    let _parse = parse(Lexer::new(text.chars()));
}

#[test]
fn test_parse_semicolon() {
    let text = ";";
    let _parse = parse(Lexer::new(text.chars()));
}

#[test]
fn test_parse_minimal_main() {
    let text = "int main() {\n    return 0;\n}\n";
    let _parse = parse(Lexer::new(text.chars()));
}

#[test]
fn test_parse_typedef() {
    let text = "typedef unsigned int blah;";
    let _parse = parse(Lexer::new(text.chars()));
}

#[test]
fn test_parse_gcd() {
    let text = include_str!("gcd.c");
    let _parse = parse(Lexer::new(text.chars()));
}

#[test]
#[ignore]
fn test_n_body() {
    let mut gcc = Command::new("gcc")
        .args(&["-E", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let input = gcc.stdin.as_mut().unwrap();

    input
        .write_all(include_str!("n-body.c").as_bytes())
        .unwrap();

    let output = gcc.wait_with_output().unwrap().stdout;

    let text = String::from_utf8(output).unwrap();

    println!("{}", text);

    println!(
        "{:#?}",
        Lexer::new(text.chars()).collect::<Vec<LexResult>>()
    );

    let _parse = parse(Lexer::new(text.chars()));
}
