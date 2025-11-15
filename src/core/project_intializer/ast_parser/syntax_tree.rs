use tree_sitter::{Language, Tree};

pub struct SyntaxTree {}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn get(&self, code: &String, language: &Language) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .expect("Error loading Python parser");
        let tree = parser.parse(&code, None).unwrap();
        return tree;
    }
}