use std::collections::HashMap;
use std::path::PathBuf;

pub struct PreprocessorFunction {
    variables: Vec<String>,
    replacement: String,
}

pub struct Preprocessor<T: Iterator<Item = char>> {
    replacements: HashMap<String, String>,
    functions: HashMap<String, PreprocessorFunction>,
    search_path: Vec<PathBuf>,
    local_path: Vec<PathBuf>,
    lookahead: Vec<char>,
    it: T,
}

impl<T: Iterator<Item = char>> Iterator for Preprocessor<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
