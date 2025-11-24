use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;
use std::io;

use tree_sitter::{Language, QueryCursor};
use tree_sitter_python;

use crate::core::ast_parser::import_classifier::ImportClassifier;
use crate::core::ast_parser::query::ParseQuery;
use crate::core::ast_parser::syntax_tree::SyntaxTree;
use crate::core::ast_parser::tree_analyzer::TreeAnalyzer;

pub struct AstParser {
    file_data: Mutex<HashMap<String, HashMap<String, Vec<String>>>>,
    import_classifier: ImportClassifier,
    query: ParseQuery,
    tree: SyntaxTree,
    tree_analyzer: TreeAnalyzer,
}

impl AstParser {

    pub fn new() -> Self {
        Self {
            file_data: Mutex::new(HashMap::new()),
            import_classifier: ImportClassifier::new(),
            query: ParseQuery::new(),
            tree: SyntaxTree::new(),
            tree_analyzer: TreeAnalyzer::new(),
        }
    }

    pub fn parse(&self, path: &Path) {
        let mut variables: HashMap<String, Vec<String>> = HashMap::new();

        let code = fs::read_to_string(path).expect("Failed to read file");
        let language: Language = tree_sitter_python::LANGUAGE.into();

        let tree = self.tree.get(&code, &language);
        let root_node = tree.root_node();
        let query = self.query.get(language);
        
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(
            &query, 
            root_node, 
            code.as_bytes()
        );
        let result = self.tree_analyzer.analyze(matches, &code, &query);
        let imports = result.0;
        let defines = result.1;
        let uses = result.2;
        
        variables.insert("imports".to_string(), imports);
        variables.insert("defines".to_string(), defines);
        variables.insert("uses".to_string(), uses);

        self.file_data.lock().unwrap().insert(
            path.display().to_string(), 
            variables,
        );
    }

    pub fn write_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.file_data).unwrap();
        fs::write("./.pytrek/file_data.json", json)?;
        Ok(())
    }
}