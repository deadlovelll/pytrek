use std::{ fs, io };
use std::io::{ BufReader, Read };
use std::path::{ Path };
use std::collections::HashMap;

use regex::Regex;
use blake3;

pub struct FileHasher {
    hash_map: HashMap<String, String>
}

impl FileHasher {

    pub fn new() -> Self {
        Self { hash_map: HashMap::new() }
    }

    pub fn hash(&mut self, path: &Path) {
        let entries = fs::read_dir(path).unwrap();
        for entry in entries {
            let entry_result = entry.unwrap();
            let md = fs::metadata(entry_result.path()).unwrap();
            let is_dir = md.is_dir();
            if is_dir {
                self.hash(&entry_result.path());
            } else {
                let path_str= entry_result.path();
                let path = path_str.to_str().unwrap();
                let is_eligible = self.is_eligible(path);
                if is_eligible {
                    let file_hash = match self.get_file_hash(&entry_result.path()) {
                        Ok(hash) => hash,
                        Err(e) => {
                            eprintln!(
                                "Failed to hash file {}, error: {}", 
                                entry_result.path().display(), 
                                e
                            );
                            break;
                        }
                    };
                    self.hash_map.insert(entry_result.path().display().to_string(), file_hash);
                }
            }
        }
        self.write_to_file();
    }

    fn write_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.hash_map).unwrap();
        fs::write("./.pytrek/file_hashes.json", json);
        Ok(())
    }

    fn is_eligible(&self, path: &str) -> bool {
        let ignore_regex = Regex::new(r"(^|/)(__+).*\.py$").unwrap();
        return 
        path.ends_with(".py") 
        && !path.contains("venv")
        && !path.contains("test")
        && !ignore_regex.is_match(path)
    }

    fn get_file_hash(&self, path: &Path) -> io::Result<String> {
        let f = fs::File::open(path)?;          
        let mut reader = BufReader::new(f);
        let mut hasher = blake3::Hasher::new();
        let mut buffer = [0u8; 8 * 1024];

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }

        let output = hasher.finalize();
        Ok(output.to_hex().to_string())       
    }
}