use std::path::Path;
use std::collections::HashMap;

use crate::core::AstParser;

pub struct GraphCreator {
    ast_data: HashMap<String, String>,
    ast_parser: AstParser,
}

impl GraphCreator {
    pub fn new() -> Self {
        Self {ast_data: HashMap::new(), ast_parser: AstParser::new()}
    }

    pub fn create_graph(
        &self, 
        path: &Path,
        root_dirs: &Vec<String>,
    ) {
        self.ast_parser.parse(path, root_dirs);
    }

    pub fn write_to_file(&self) {
        self.ast_parser.write_to_file();
    }
}