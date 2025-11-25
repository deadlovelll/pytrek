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