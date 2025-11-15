use tree_sitter::{Language, Tree};

/// `SyntaxTree` is a utility for parsing source code into a `tree_sitter::Tree`.
/// It provides methods to create a parser and generate the syntax tree
/// for a given language.
pub struct SyntaxTree {}

impl SyntaxTree {

    /// Creates a new instance of `SyntaxTree`.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree_util = SyntaxTree::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
    
    /// Parses the given source `code` using the specified `language`
    /// and returns the resulting `tree_sitter::Tree`.
    ///
    /// # Arguments
    ///
    /// * `code` - A reference to a `String` containing the source code to parse.
    /// * `language` - The `tree_sitter::Language` to use for parsing.
    ///
    /// # Returns
    ///
    /// A `Tree` representing the syntax tree of the input code.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The parser fails to load the specified language.
    /// - The parser fails to parse the code (`unwrap()` on `parse` result).
    ///
    /// # Examples
    ///
    /// ```
    /// let code = "import os\nprint(os.path)";
    /// let language = tree_sitter_python::LANGUAGE;
    /// let tree_util = SyntaxTree::new();
    /// let tree = tree_util.get(&code.to_string(), &language);
    /// ```
    pub fn get(&self, code: &String, language: &Language) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .expect("Error loading Python parser");
        let tree = parser.parse(&code, None).unwrap();
        return tree;
    }
}