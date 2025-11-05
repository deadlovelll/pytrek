use crate::core::file_hasher::FileHasher;
use crate::core::graph_creator::GraphCreator;

pub struct ProjectInitializer {
    file_hasher: FileHasher,
    graph_creator: GraphCreator,
}

impl ProjectInitializer {
    pub fn new(file_hasher: FileHasher, graph_creator: GraphCreator) -> Self {
        Self { file_hasher, graph_creator }
    }

    pub fn init(&self) {
        self.file_hasher.hash();
        self.graph_creator.create_graph();
    }
}