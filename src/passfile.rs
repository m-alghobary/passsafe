use std::{
    fs::{File, OpenOptions},
    io::{BufRead, Read, Seek, SeekFrom, Write},
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

    pub fn update_pass(&self, pass_line: &Passline) -> std::io::Result<()> {
        let mut file = self.open_to_write()?;

        let mut old_content = String::new();
        file.read_to_string(&mut old_content)?;

        let mut new_content = String::new();
        for line in old_content.lines() {
            if line.starts_with(&format!("{}: ", pass_line.name)) {
                new_content =
                    old_content.replace(&format!("{}\n", line), pass_line.format().as_str());
                break;
            }
        }

        self.overwrite_content(&mut file, &new_content)?;
        Ok(())
    }

    pub fn get_pass_line(&self, pass_name: &String) -> std::io::Result<Option<Passline>> {
        let path = Path::new(&self.path);

        let pass_file = std::fs::File::open(path)?;
        let file_reader = std::io::BufReader::new(pass_file);

        for line in file_reader.lines() {
            let line = line.unwrap_or_default();

            if line.starts_with(pass_name) {
                return Ok(Some(self.parse_pass_line(line)));
            }
        }

        Ok(None)
    }

    pub fn delete_pass_line(&self, pass_name: &String) -> std::io::Result<bool> {
        let mut file = self.open_to_write()?;

        let mut old_content = String::new();
        file.read_to_string(&mut old_content)?;

        let mut new_content = String::new();
        for line in old_content.lines() {
            if line.starts_with(&format!("{}: ", pass_name)) {
                new_content = old_content.replace(&format!("{}\n", line), "");
                break;
            }
        }

        self.overwrite_content(&mut file, &new_content)?;
        Ok(true)
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

    pub fn get_pass_lines(&self) -> std::io::Result<Vec<Passline>> {
        let path = Path::new(&self.path);

        let pass_file = std::fs::File::open(path)?;
        let file_reader = std::io::BufReader::new(pass_file);

        let result: Vec<Passline> = file_reader
            .lines()
            .into_iter()
            .map(|line| {
                let line = line.unwrap_or_default();
                self.parse_pass_line(line)
            })
            .collect();

        Ok(result)
    }

    fn overwrite_content(&self, file: &mut File, new_content: &String) -> std::io::Result<()> {
        file.seek(SeekFrom::Start(0))?;
        file.set_len(new_content.len() as u64)?;
        assert_eq!(file.write(&new_content.as_bytes())?, new_content.len());

        Ok(())
    }

    fn open_to_write(&self) -> std::io::Result<File> {
        let path = Path::new(&self.path);

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
    }

    fn parse_pass_line(&self, line: String) -> Passline {
        let mut parts = line.split(":");
        let name = parts.next().unwrap_or_default().trim().to_string();
        let pass = parts.next().unwrap_or_default().trim().to_string();

        Passline::new(name, pass)
    }
}
