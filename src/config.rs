use std::fs::File;

pub struct Config {
    file: File,
}

impl Config {
    pub fn new(path: &str) -> std::io::Result<Self> {
        Ok(Self {
            file: File::create(path)?,
        })
    }
}
