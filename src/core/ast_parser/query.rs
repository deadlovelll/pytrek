use tree_sitter::{Query, Language};

pub struct ParseQuery {}

impl ParseQuery {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, language: Language) -> Query {
        let query_src = r#"
            (import_statement (dotted_name) @import)
            (import_from_statement
                module_name: (dotted_name) @module
                name: (dotted_name) @name
            )
            (function_definition
                name: (identifier) @function
            )
        "#;
        let query = Query::new(&language, query_src).unwrap();
        return query;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use tree_sitter::QueryCursor;
    use tree_sitter::StreamingIterator;

    #[test]
    fn test_gets_query() {
        let query = ParseQuery::new();
        let language: Language = tree_sitter_python::LANGUAGE.into();
        let results = query.get(language);
        let _: Query = results;
    }

    #[test]
    fn test_query_returns_imports_mix() {
        let code = r#"
            import os
            from sys import path
            import fastapi
            from service.something import Something
        "#;

        let query = ParseQuery::new();
        let language: Language = tree_sitter_python::LANGUAGE.into();

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .expect("Error loading Python parser");
        
        let tree = parser.parse(&code, None).unwrap();
        let root_node = tree.root_node();
        let results = query.get(language);
        
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(
            &results, 
            root_node, 
            code.as_bytes()
        );

        let mut seen: HashSet<&str> = HashSet::new();
        
        while let Some(m) = matches.next() {
            for cap in m.captures.iter() {
                let name = results.capture_names()[cap.index as usize];
                println!("{}", name);
                seen.insert(name);
            }
        }
        assert_eq!(seen.contains("import"), true);
        assert_eq!(seen.contains("module"), true);
        assert_eq!(seen.contains("name"), true);

    }

    #[test]
    fn test_query_returns_imports() {
        let code = r#"
            import os
            import sys
            import fastapi
            import settings
        "#;

        let query = ParseQuery::new();
        let language: Language = tree_sitter_python::LANGUAGE.into();

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .expect("Error loading Python parser");
        
        let tree = parser.parse(&code, None).unwrap();
        let root_node = tree.root_node();
        let results = query.get(language);
        
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(
            &results, 
            root_node, 
            code.as_bytes()
        );

        let mut seen: HashSet<&str> = HashSet::new();
        
        while let Some(m) = matches.next() {
            for cap in m.captures.iter() {
                let name = results.capture_names()[cap.index as usize];
                println!("{}", name);
                seen.insert(name);
            }
        }
        assert_eq!(seen.contains("import"), true);
        assert_eq!(seen.contains("module"), false);
        assert_eq!(seen.contains("name"), false);
    }

    #[test]
    fn test_query_returns_imports_from() {
        let code = r#"
            from os import osos
            from sys import syscall
            from fastapi import ApiRouter
            from settings import KafkaDsn
        "#;

        let query = ParseQuery::new();
        let language: Language = tree_sitter_python::LANGUAGE.into();

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .expect("Error loading Python parser");
        
        let tree = parser.parse(&code, None).unwrap();
        let root_node = tree.root_node();
        let results = query.get(language);
        
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(
            &results, 
            root_node, 
            code.as_bytes()
        );

        let mut seen: HashSet<&str> = HashSet::new();
        
        while let Some(m) = matches.next() {
            for cap in m.captures.iter() {
                let name = results.capture_names()[cap.index as usize];
                println!("{}", name);
                seen.insert(name);
            }
        }
        assert_eq!(seen.contains("import"), false);
        assert_eq!(seen.contains("module"), true);
        assert_eq!(seen.contains("name"), true);
    }
}