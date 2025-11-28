use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use regex::Regex;
use rayon::prelude::*;

use crate::core::FileHasher;
use crate::core::GraphCreator;

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
        let root_dirs = self.get_root_dirs(path);

        self.walk(path, &root_dirs);
        self.graph_creator.write_to_file();
    }

    pub fn walk(&self, path: &Path, root_dirs: &Vec<String>) {
        let entries = self.get_entries(path);
        entries.par_iter().for_each(|entry| {
            if entry.file_type().unwrap().is_dir() {
                self.walk(&entry.path(), root_dirs);
            } else if self.is_eligible(entry.path().to_str().unwrap()) {
                if !entry.path().to_str().unwrap().contains("test") {
                    self.file_hasher.hash(&entry.path());
                }
                self.graph_creator.create_graph(&entry.path(), &root_dirs);
            }
        });
    }

    fn get_entries(&self, path: &Path) -> Vec<DirEntry> {
        let entries: Vec<_> = fs::read_dir(path)
            .unwrap()
            .map(|e| e.unwrap())
            .collect();

        return entries
    }

    fn is_eligible(&self, path: &str) -> bool {
        let ignore_regex = Regex::new(r"(^|/)(__+).*\.py$").unwrap();
        return path.ends_with(".py") 
        && !path.contains("venv")
        && !path.contains("test")
        && !ignore_regex.is_match(path)
    }

    fn is_eligible_dir(&self, path: &str) -> bool {
        return !path.contains("cache")       
        && !path.starts_with("__")   
        && !path.ends_with("__")     
        && !path.contains("venv")      
        && !path.starts_with('.')  
    }

    fn get_root_dirs(&self, path: &Path) -> Vec<String> {
        let mut root_dirs = vec![];
        let entries = self.get_entries(path);
        for root_entry in entries {
            if root_entry.path().is_dir() {
                if self.is_eligible_dir(&root_entry.path().to_str().unwrap()[2..]) {
                    if let Some(name) = root_entry.path().file_name() {
                        let name = name.to_string_lossy().to_string();
                        root_dirs.push(name);
                    }
                }
            }
        }
        return root_dirs
    }
}