use std::io;
use std::path::{Path};

use crate::core::FileWalker;
use crate::core::ConfigCreator;

/// `ProjectInitializer` is responsible for setting up a project.
///
/// Responsibilities:
/// - Generate configuration files via `ConfigCreator`.
/// - Traverse the project directory and build file 
/// hashes and a dependency graph via `FileWalker`.
pub struct ProjectInitializer {
    /// Handles file traversal, hash calculation, 
    /// and dependency graph creation.
    file_walker: FileWalker,
    /// Handles project configuration creation.
    config_creator: ConfigCreator,
}

impl ProjectInitializer {
    
    /// Creates a new `ProjectInitializer`.
    ///
    /// # Parameters
    /// - `file_walker`: The `FileWalker` instance for 
    /// directory traversal, hash calculation, 
    /// and dependency graph creation.
    /// - `config_creator`: The `ConfigCreator` 
    /// instance for generating project configuration files.
    ///
    /// # Returns
    /// A new instance of `ProjectInitializer`.
    pub fn new(
        file_walker: FileWalker,  
        config_creator: ConfigCreator
    ) -> Self {
        Self { file_walker, config_creator }
    }

    /// Initializes the project.
    ///
    /// Steps performed:
    /// 1. Generates configuration files using `ConfigCreator`.
    /// 2. Walks the project directory starting from `./`, 
    /// building file hashes and a dependency graph via `FileWalker`.
    ///
    /// # Errors
    /// Returns an `io::Error` if configuration creation fails.
    pub fn init(&mut self) -> io::Result<()> {
        self.config_creator.create_config()?;
        self.file_walker.run(Path::new("."));
        Ok(())
    }
}