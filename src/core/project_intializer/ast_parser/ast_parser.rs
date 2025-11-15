use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;
use std::io;

use tree_sitter::{Language, QueryCursor};
use tree_sitter_python;
use tree_sitter::StreamingIterator;

use crate::core::project_intializer::ast_parser::import_classifier::ImportClassifier;
use crate::core::project_intializer::ast_parser::query::ParseQuery;
use crate::core::project_intializer::ast_parser::syntax_tree::SyntaxTree;

pub struct AstParser {
    file_data: Mutex<HashMap<String, HashMap<String, Vec<String>>>>,
    import_classifier: ImportClassifier,
    query: ParseQuery,
    tree: SyntaxTree,
}

impl AstParser {

    pub fn new() -> Self {
        Self {
            file_data: Mutex::new(HashMap::new()),
            import_classifier: ImportClassifier::new(),
            query: ParseQuery::new(),
            tree: SyntaxTree::new(),
        }
    }

    pub fn parse(&self, path: &Path) {
        let mut imports: Vec<String> = vec![];
        let mut defines: Vec<String> = vec![];
        let mut uses: Vec<String> = vec![];
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
                let is_eligible: bool = self.import_classifier.is_eligible(&m);
                if is_eligible {
                    for n in names {
                        let full_import = format!("{}.{}", m, n);
                        imports.push(full_import);
                    }
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