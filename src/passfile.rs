use std::{fs::OpenOptions, io::Write, path::Path};

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
}
