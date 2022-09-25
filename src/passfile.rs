use std::{
    fs::OpenOptions,
    io::{BufRead, Write},
    path::Path,
};

use crate::Passline;

pub struct PassFile {
    path: String,
}

impl PassFile {
    pub fn new() -> Self {
        Self {
            path: String::from("./passfile"),
        }
    }

    pub fn write_pass(&self, pass_line: &Passline) -> std::io::Result<()> {
        let formatted_line = pass_line.format();

        let path = Path::new(&self.path);
        let mut file = OpenOptions::new().append(true).create(true).open(path)?;

        file.write_all(&formatted_line.as_bytes())?;

        Ok(())
    }

    pub fn pass_line_exist(&self, pass_name: &String) -> std::io::Result<bool> {
        let path = Path::new(&self.path);

        if !path.exists() {
            return Ok(false);
        }

        let pass_file = std::fs::File::open(path)?;
        let file_reader = std::io::BufReader::new(pass_file);

        let mut exist = false;
        for line in file_reader.lines() {
            let line = line.unwrap_or_default();

            if line.starts_with(pass_name) {
                exist = true;
                break;
            }
        }

        Ok(exist)
    }
}
