pub struct FileHasher {
    hashing_alg: String,
}

impl FileHasher {
    pub fn new(hashing_alg) -> Self {
        Self { hashing_alg }
    }

    pub hash(&self) {
        println!("Hello, {}!", self.hashing_alg);
    }
}