use std::path::Path;
use std::collections::HashMap;

pub struct GraphCreator {
    ast_data: HashMap<String, String>
}

impl GraphCreator {
    pub fn new() -> Self {
        Self {ast_data: HashMap::new()}
    }

    pub fn create_graph(&self, path: &Path) {
        print!("hello");
    }
}