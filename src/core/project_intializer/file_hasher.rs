use std::{ fs, io };
use std::io::{ BufReader, Read };
use std::path::{ Path };
use std::collections::HashMap;
use std::sync::Mutex;

use blake3::{self};

pub struct FileHasher {
    hash_map: Mutex<HashMap<String, String>>,
}

impl FileHasher {

    pub fn new() -> Self {
        Self { hash_map: Mutex::new(HashMap::new())}
    }

    pub fn hash(&self, path: &Path) -> io::Result<()> {
        match self.get_file_hash(path) {
            Ok(file_hash) => {
                self.hash_map.lock().unwrap().insert(
                    path.display().to_string(), 
                    file_hash
                );
            }
            Err(e) => eprintln!(
                "Failed to hash file {}: {}", 
                path.display(), 
                e
            ),
        }
        Ok(())
    }

    pub fn write_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.hash_map).unwrap();
        fs::write("./.pytrek/file_hashes.json", json)?;
        Ok(())
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