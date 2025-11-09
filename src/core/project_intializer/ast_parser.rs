use std::{fs};
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;
use std::io;

use tree_sitter::{Language, Query, QueryCursor};
use tree_sitter_python;
use tree_sitter::StreamingIterator;

pub struct AstParser {
    file_data: Mutex<HashMap<String, Vec<String>>>
}

impl AstParser {

    pub fn new() -> Self {
        Self {file_data: Mutex::new(HashMap::new())}
    }

    pub fn parse(&self, path: &Path) {
        let code = fs::read_to_string(path).expect("Failed to read file");
        let mut parser = tree_sitter::Parser::new();
        let language: Language = tree_sitter_python::LANGUAGE.into();
        parser
            .set_language(&language)
            .expect("Error loading Python parser");
        let tree = parser.parse(&code, None).unwrap();
        let root_node = tree.root_node();
        let query_src = r#"
            (import_statement name: (dotted_name) @import)
            (import_from_statement module_name: (dotted_name) @from_import)
        "#;
        let query = Query::new(&language, query_src).unwrap();
        let mut cursor = QueryCursor::new();
        let mut imports = vec![];
        let mut matches = cursor.matches(
            &query, 
            root_node, 
            code.as_bytes()
        );
        while let Some(m) = matches.next() {
            for cap in m.captures.iter() {
                let text = cap.node.utf8_text(code.as_bytes())
                .unwrap()
                .to_string();
                let name = query.capture_names()[cap.index as usize];
                match name {
                    "imports" | "from import" => imports.push(text),
                    _ => {},
                }
            }
        }
        self.file_data.lock().unwrap().insert(
            path.display().to_string(), 
            imports,
        );
    }

    pub fn write_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.file_data).unwrap();
        fs::write("./.pytrek/file_data.json", json)?;
        Ok(())
    }
}