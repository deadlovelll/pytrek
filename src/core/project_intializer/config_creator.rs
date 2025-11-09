use std::{fs::{self, OpenOptions}, io::{self, Write}};
use std::path::Path;

pub struct ConfigCreator {}

impl ConfigCreator {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_config(&self) -> io::Result<()>{
        self.create_toml_config()?;
        self.create_config_directory()?;
        Ok(())
    }

    fn create_toml_config(&self) -> io::Result<()> {
        let file_exists = Path::new("pyproject.toml").exists();
        if file_exists {
            self.write_toml_config();
        } else {
            fs::File::create("pyproject.toml")?;
            self.write_toml_config();
        }
        Ok(())
    }

    fn write_toml_config(&self) {
        let mut data_file = OpenOptions::new()
            .append(true)
            .open("pyproject.toml")
            .expect("Cannot open a file");

        data_file
            .write_all(b"[tool.pytrek]\npath = '.'")
            .expect("Cannot write to file");
    }

    fn create_config_directory(&self) -> io::Result<()> {
        fs::create_dir(".pytrek/")?;
        fs::File::create(".pytrek/file_hashes.json")?;
        fs::File::create(".pytrek/graph.json")?;
        Ok(())
    }
}