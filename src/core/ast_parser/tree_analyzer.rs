use tree_sitter::{Query, QueryMatches};

use crate::core::project_intializer::ast_parser::import_classifier::ImportClassifier;
use crate::core::project_intializer::ast_parser::dot_name::DotName;

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
        &mut self,
        matches: &mut QueryMatches,
        code: &str,
        query: &Query,
        imports: &mut Vec<String>,
        defines: &Vec<String>,
        uses: &Vec<String>,
    ) {
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
    }
}