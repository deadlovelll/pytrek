use std::{ fs, io };
use std::io::{ BufReader, Read };
use std::path::{ Path };
use std::collections::HashMap;
use std::sync::Mutex;

use blake3::{self};

pub struct FileHasher {
    // Stores file paths and their corresponding hashes.
    // Wrapped in a Mutex so the type is safe to use from multiple threads.
    hash_map: Mutex<HashMap<String, String>>,
}

impl FileHasher {

    /// Creates a new `FileHasher` with an empty hash map.
    pub fn new() -> Self {
        Self { hash_map: Mutex::new(HashMap::new())}
    }

    /// Computes the BLAKE3 hash of the given file and stores it in the
    /// internal `HashMap`, where the key is the file path and the value is
    /// the hex-encoded hash.
    ///
    /// If hashing fails, the error is printed to stderr. The function always
    /// returns `Ok(())`.
    pub fn hash(&self, path: &Path) -> io::Result<()> {
        match self.get_file_hash(path) {
            Ok(file_hash) => {
                // Insert the hash for this path.
                self.hash_map.lock().unwrap().insert(
                    path.display().to_string(), 
                    file_hash
                );
            }
            // Log the error but don't stop the process.
            Err(e) => eprintln!(
                "Failed to hash file {}: {}", 
                path.display(), 
                e
            ),
        }
        Ok(())
    }

    /// Serializes the internal hash map to JSON and writes it to
    /// `./.pytrek/file_hashes.json`.
    ///
    /// Returns an I/O error if writing the file fails.
    pub fn write_to_file(&self) -> io::Result<()> {
        // Convert the mutex-protected HashMap into pretty-printed JSON.
        let json = serde_json::to_string_pretty(&self.hash_map).unwrap();

        // Save the JSON to the .pytrek directory.
        fs::write("./.pytrek/file_hashes.json", json)?;
        Ok(())
    }

    /// Computes and returns the BLAKE3 hash of the file contents.
    ///
    /// The file is streamed in chunks rather than read fully into memory.
    /// Returns the hex-encoded hash as a `String`.
    fn get_file_hash(&self, path: &Path) -> io::Result<String> {
        // Open the file for reading.
        let f = fs::File::open(path)?;
        let mut reader = BufReader::new(f);

        // Initialize the BLAKE3 hasher.
        let mut hasher = blake3::Hasher::new();

        // Reusable 8 KB buffer to avoid allocating per read.
        let mut buffer = [0u8; 8192];

        // Read the file in chunks until EOF.
        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 { break; }          // EOF
            hasher.update(&buffer[..n]);  // Feed chunk into the hasher
        }

        // Finalize the hash and return hex-encoded string.
        let output = hasher.finalize();
        Ok(output.to_hex().to_string())     
    }
}