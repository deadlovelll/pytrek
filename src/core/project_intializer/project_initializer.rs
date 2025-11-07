use std::io;
use std::path::{Path};
use std::time::Instant;

use crate::core::project_intializer::file_hasher::FileHasher;
use crate::core::project_intializer::graph_creator::GraphCreator;
use crate::core::project_intializer::config_creator::ConfigCreator;

pub struct ProjectInitializer {
    file_hasher: FileHasher,
    graph_creator: GraphCreator,
    config_creator: ConfigCreator,
}

impl ProjectInitializer {
    pub fn new(
        file_hasher: FileHasher, 
        graph_creator: GraphCreator, 
        config_creator: ConfigCreator
    ) -> Self {
        Self { file_hasher, graph_creator, config_creator }
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.config_creator.create_config()?;
        self.file_hasher.hash(Path::new("./"))?;
        self.graph_creator.create_graph();
        Ok(())
    }
}