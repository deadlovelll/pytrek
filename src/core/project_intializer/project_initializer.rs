use std::io;
use std::path::{Path};

use crate::core::FileWalker;
use crate::core::ConfigCreator;

pub struct ProjectInitializer {
    file_walker: FileWalker,
    config_creator: ConfigCreator,
}

impl ProjectInitializer {
    
    pub fn new(
        file_walker: FileWalker,  
        config_creator: ConfigCreator
    ) -> Self {
        Self { file_walker, config_creator }
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.config_creator.create_config()?;
        self.file_walker.run(Path::new("./"));
        Ok(())
    }
}