pub struct FileHasher {
    hashing_alg: String,
}

impl FileHasher {
    pub fn new(hashing_alg: String) -> Self {
        Self { hashing_alg }
    }

    pub fn hash(&self) {
        println!("Hello, {}!", self.hashing_alg);
    }
}