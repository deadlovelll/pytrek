/// `DotName` is a utility for extracting dotted names or text fragments
/// from a `tree_sitter::Node` in a source code string.
pub struct DotName {}

impl DotName {

    /// Creates a new instance of `DotName`.
    ///
    /// # Examples
    ///
    /// ```
    /// let extractor = DotName::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Extracts the text corresponding to a given `tree_sitter::Node`
    /// from the source `code`.
    ///
    /// # Arguments
    ///
    /// * `code` - A string slice containing the source code.
    /// * `node` - The `tree_sitter::Node` representing the AST node to extract.
    ///
    /// # Returns
    ///
    /// A `String` containing the text of the node.
    ///
    /// # Panics
    ///
    /// This function will panic if `node.utf8_text` returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// let code = "import os";
    /// let node = ...; // a tree_sitter::Node representing `os`
    /// let text = extractor.get(code, node);
    /// assert_eq!(text, "os");
    /// ```
    pub fn get(&self, code: &str, node: tree_sitter::Node) -> String {
        let text = node.utf8_text(code.as_bytes())
        .unwrap()
        .to_string();
        return text;
    }
}