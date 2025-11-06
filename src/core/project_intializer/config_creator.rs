use std::{fs, io};

pub struct ConfigCreator {}

impl ConfigCreator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_config(&self) -> io::Result<()>{
        self.create_toml_config();
        self.create_config_directory()?;
        Ok(())
    }

    fn create_toml_config(&self) {
        println!("hello");
    }

    fn create_config_directory(&self) -> std::io::Result<()> {
        fs::create_dir(".pytrek/")?;
        fs::File::create(".pytrek/file_hashes.json")?;
        fs::File::create(".pytrek/graph.json")?;
        Ok(())
    }
}