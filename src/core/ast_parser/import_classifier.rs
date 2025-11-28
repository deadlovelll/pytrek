/// Classifies Python imports as standard library or third-party/project-specific.
pub struct ImportClassifier {
}

impl ImportClassifier {
    /// Creates a new `ImportClassifier` and initializes the
    /// list of standard library modules.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns `true` if the import is project level.
    ///
    /// # Arguments
    ///
    /// * `import` - The module name to classify.
    ///
    /// # Example
    ///
    /// ```
    /// let classifier = ImportClassifier::new();
    /// assert_eq!(classifier.is_eligible(&"infra.db.modules".to_string()), true);
    /// assert_eq!(classifier.is_eligible(&"os".to_string()), false);
    /// ```
    pub fn is_eligible(
        &self, 
        import: &String,
        root_dirs: &Vec<String>,
    ) -> bool {

        for dir in root_dirs {
            if import.starts_with(dir) {
                return true;
            }
        }
        return false;
    }
}