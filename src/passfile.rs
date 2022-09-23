use std::{fs::OpenOptions, io::Write, path::Path};

pub struct PassFile {
    path: String,
}

impl PassFile {
    pub fn new() -> Self {
        Self {
            path: String::from("./passfile"),
        }
    }

    pub fn write_pass(&self, name: &String, password: &String) -> std::io::Result<()> {
        let pass_buf = String::from(format!("{0}: {1}\n", name, password));

        let path = Path::new(&self.path);
        let mut file = OpenOptions::new().append(true).create(true).open(path)?;

        file.write_all(&pass_buf.as_bytes())?;

        Ok(())
    }
}
