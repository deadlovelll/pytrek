use std::{fs, io::Read};
use std::path::Path;

use tree_sitter::{Parser, TreeCursor};
use tree_sitter_python;

pub struct AstParser {}

impl AstParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, path: &Path) {
        let code = fs::read_to_string(path).expect("Failed to read file");
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_python::LANGUAGE;
        parser
            .set_language(&language.into())
            .expect("Error loading Python parser");
        let tree = parser.parse(code, None).unwrap();
    }
}