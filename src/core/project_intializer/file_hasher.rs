use std::{ fs, io };
use std::io::{ BufReader, Read };
use std::path::{ Path };
use std::collections::HashMap;
use std::sync::Mutex;

use regex::Regex;
use blake3::{self};
use rayon::prelude::*;

pub struct FileHasher {
    hash_map: Mutex<HashMap<String, String>>,
}

impl FileHasher {

    pub fn new() -> Self {
        Self { hash_map: Mutex::new(HashMap::new())}
    }

    pub fn hash(&self, path: &Path) -> io::Result<()> {
        self.walk(path);
        self.write_to_file()?;
        Ok(())
    }

    fn walk(&self, path: &Path) {
        let entries: Vec<_> = fs::read_dir(path)
            .unwrap()
            .map(|e| e.unwrap())
            .collect();

        entries.par_iter().for_each(|entry| {
            if entry.file_type().unwrap().is_dir() {
                self.walk(&entry.path());
            } else if self.is_eligible(entry.path().to_str().unwrap()) {
                match self.get_file_hash(&entry.path()) {
                    Ok(file_hash) => {
                        self.hash_map.lock().unwrap().insert(
                            entry.path().display().to_string(), 
                            file_hash
                        );
                    }
                    Err(e) => eprintln!(
                        "Failed to hash file {}: {}", 
                        entry.path().display(), 
                        e
                    ),
                }
            }
        });
    }

    fn write_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.hash_map).unwrap();
        fs::write("./.pytrek/file_hashes.json", json)?;
        Ok(())
    }

    fn is_eligible(&self, path: &str) -> bool {
        let ignore_regex = Regex::new(r"(^|/)(__+).*\.py$").unwrap();
        return path.ends_with(".py") 
        && !path.contains("venv")
        && !path.contains("test")
        && !ignore_regex.is_match(path)
    }

    fn get_file_hash(&self, path: &Path) -> io::Result<String> {
        let f = fs::File::open(path)?;          
        let mut reader = BufReader::new(f);
        let mut hasher = blake3::Hasher::new();
        let mut buffer = [0u8; 8192];

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }

        let output = hasher.finalize();
        Ok(output.to_hex().to_string())       
    }
}