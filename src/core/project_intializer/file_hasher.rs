use std::{fs, io};
use std::io::{BufReader, Read};
use std::path::{ Path };

use blake3;

pub struct FileHasher {
    hashing_alg: String,
}

impl FileHasher {
    pub fn new(hashing_alg: String) -> Self {
        Self { hashing_alg }
    }

    pub fn hash(&self, path: &Path) {
        let entries = fs::read_dir(path).unwrap();
        for entry in entries {
            let entry_result = entry.unwrap();
            let md = fs::metadata(entry_result.path()).unwrap();
            let is_dir = md.is_dir();
            if is_dir {
                self.hash(&entry_result.path());
            } else {
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
                println!("path is {}, hash is {}", entry_result.path().display(), file_hash);
            }
        }
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