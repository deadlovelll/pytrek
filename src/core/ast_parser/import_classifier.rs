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
            if import.starts_with(dir) || import.starts_with(".") {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_is_local() {
        let root_dirs = vec![
            "application".to_string(), 
            "services".to_string()
        ];
        let import = "services.foo.Bar".to_string();

        let classifier = ImportClassifier::new();
        let result = classifier.is_eligible(&import, &root_dirs);

        assert_eq!(result, true);
    }

    #[test]
    fn test_import_is_not_local() {
        let root_dirs = vec![
            "application".to_string(), 
            "services".to_string()
        ];
        let import = "fastapi.ApiRouter".to_string();

        let classifier = ImportClassifier::new();
        let result = classifier.is_eligible(&import, &root_dirs);

        assert_eq!(result, false);
    }

    #[test]
    fn test_import_is_relative() {
        let root_dirs = vec![
            "application".to_string(), 
            "services".to_string()
        ];
        let import = "...services.global.GlobalService".to_string();

        let classifier = ImportClassifier::new();
        let result = classifier.is_eligible(&import, &root_dirs);

        assert_eq!(result, true);
    }
}