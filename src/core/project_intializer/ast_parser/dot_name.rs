pub struct DotName {}

impl DotName {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, code: &str, node: tree_sitter::Node) -> String {
        let text = node.utf8_text(code.as_bytes())
        .unwrap()
        .to_string();
        return text;
    }
}