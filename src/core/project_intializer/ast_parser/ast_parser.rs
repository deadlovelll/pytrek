use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;
use std::io;

use tree_sitter::{Language, Query, QueryCursor};
use tree_sitter_python;
use tree_sitter::StreamingIterator;

use crate::core::project_intializer::ast_parser::import_classifier::ImportClassifier;

pub struct AstParser {
    file_data: Mutex<HashMap<String, HashMap<String, Vec<String>>>>,
    import_classifier: ImportClassifier,
}

impl AstParser {

    pub fn new() -> Self {
        Self {
            file_data: Mutex::new(HashMap::new()),
            import_classifier: ImportClassifier::new(),
        }
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
            (import_statement (dotted_name) @import)
            (import_from_statement
                module_name: (dotted_name) @module
                name: (dotted_name) @name
            )
        "#;
        let query = Query::new(&language, query_src).unwrap();
        let mut cursor = QueryCursor::new();
        let mut imports: Vec<String> = vec![];
        let mut defines: Vec<String> = vec![];
        let mut uses: Vec<String> = vec![];
        let mut matches = cursor.matches(
            &query, 
            root_node, 
            code.as_bytes()
        );
        let mut variables: HashMap<String, Vec<String>> = HashMap::new();
        while let Some(m) = matches.next() {
            let mut module: Option<String> = None;
            let mut names: Vec<String> = Vec::new();
            for cap in m.captures.iter() {
                let name = query.capture_names()[cap.index as usize];
                match name {
                    "module" => {
                        module = Some(self.dotted_name(&code, cap.node));
                    }
                    "name" => {
                        names.push(self.dotted_name(&code, cap.node));
                    }
                    "import" => {
                        let text = self.dotted_name(&code, cap.node);
                        let is_eligible: bool = self.import_classifier.is_eligible(&text);
                        if is_eligible {
                            imports.push(text);
                        }
                    }
                    _ => {}
                }
            }
            if let Some(m) = module {
                for n in names {
                    let full_import = format!("{}.{}", m, n);
                    imports.push(full_import);
                }
            }
        }
        variables.insert("imports".to_string(), imports);
        variables.insert("defines".to_string(), defines);
        variables.insert("uses".to_string(), uses);

        self.file_data.lock().unwrap().insert(
            path.display().to_string(), 
            variables,
        );
    }

    fn dotted_name(&self, code: &str, node: tree_sitter::Node) -> String {
        let text = node.utf8_text(code.as_bytes())
        .unwrap()
        .to_string();
        return text;
    }

    pub fn write_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.file_data).unwrap();
        fs::write("./.pytrek/file_data.json", json)?;
        Ok(())
    }
}