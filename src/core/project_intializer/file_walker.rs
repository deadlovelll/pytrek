use std::fs;
use std::path::Path;

use regex::Regex;
use rayon::prelude::*;

use crate::core::project_intializer::file_hasher::FileHasher;
use crate::core::project_intializer::graph_creator::GraphCreator;

pub struct FileWalker {
    file_hasher: FileHasher,
    graph_creator: GraphCreator,
}

impl FileWalker {
    
    pub fn new() -> Self {
        Self {
            file_hasher: FileHasher::new(),
            graph_creator: GraphCreator::new(),
        }
    }

    pub fn run(&self, path: &Path) {
        self.walk(path);
        self.file_hasher.write_to_file();
        self.graph_creator.write_to_file();
    }

    pub fn walk(&self, path: &Path) {
        let entries: Vec<_> = fs::read_dir(path)
            .unwrap()
            .map(|e| e.unwrap())
            .collect();

        entries.par_iter().for_each(|entry| {
            if entry.file_type().unwrap().is_dir() {
                self.walk(&entry.path());
            } else if self.is_eligible(entry.path().to_str().unwrap()) {
                if !entry.path().to_str().unwrap().contains("test") {
                    self.file_hasher.hash(&entry.path());
                }
                self.graph_creator.create_graph(&entry.path());
            }
        });
    }

    fn is_eligible(&self, path: &str) -> bool {
        let ignore_regex = Regex::new(r"(^|/)(__+).*\.py$").unwrap();
        return path.ends_with(".py") 
        && !path.contains("venv")
        && !path.contains("test")
        && !ignore_regex.is_match(path)
    }
}