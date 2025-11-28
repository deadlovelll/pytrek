use tree_sitter::{Query, Node, QueryCursor, StreamingIterator};

use crate::core::ast_parser::import_classifier::ImportClassifier;
use crate::core::ast_parser::dot_name::DotName;

pub struct TreeAnalyzer {
    import_classifier: ImportClassifier,
    dot_name: DotName,
}

impl TreeAnalyzer {
    pub fn new() -> Self {
        Self {
            import_classifier: ImportClassifier::new(),
            dot_name: DotName::new(),
        }
    }

    pub fn analyze(
        &self,
        code: &str,
        query: &Query,
        root_node: Node,
        root_dirs: &Vec<String>,
    ) -> (Vec<String>, Vec<String>, Vec<String>) {

        let mut imports: Vec<String> = vec![];
        let mut defines: Vec<String> = vec![];
        let mut uses: Vec<String> = vec![];
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
                        module = Some(self.dot_name.get(&code, cap.node));
                    }
                    "name" => {
                        names.push(self.dot_name.get(&code, cap.node));
                    }
                    "import" => {
                        let text = self.dot_name.get(&code, cap.node);
                        let is_eligible: bool = self.import_classifier.is_eligible(&text, root_dirs);
                        if is_eligible {
                            imports.push(text);
                        }
                    }
                    "function" => {
                        let text = self.dot_name.get(&code, cap.node);
                        if !text.starts_with("__") & !text.ends_with("__") {
                            let mut parent = cap.node.parent();
                            let mut found_class = false;
                            while let Some(p) = parent {
                                if p.kind() == "class_definition" {
                                    if let Some(name_node) = p.child_by_field_name("name") {
                                        let class = name_node.utf8_text(code.as_bytes())
                                        .unwrap()
                                        .to_string();
                                        let class_method = format!("{class}.{text}");
                                        defines.push(class_method);
                                        found_class = true;
                                    }
                                }
                                parent = p.parent();
                            }
                            if !found_class {
                                defines.push(text);
                            }
                        }
                    }
                    _ => {}
                }
            }
            if let Some(m) = module {
                let is_eligible: bool = self.import_classifier.is_eligible(&m, root_dirs);
                if is_eligible {
                    for n in names {
                        let full_import = format!("{}.{}", m, n);
                        imports.push(full_import);
                    }
                }
            }
        }
        return (imports, defines, uses)
    }
}