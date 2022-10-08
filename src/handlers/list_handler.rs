use std::io;

use crate::passfile::PassFile;

pub struct ListOptions {
    pub show_password: bool,
}

pub struct ListHandler;

impl ListHandler {
    pub fn handle(options: ListOptions) -> io::Result<()> {
        let pass_file = PassFile::new();

        let passlines = pass_file.get_pass_lines()?;

        println!();
        println!("Name\t\tPassword");
        println!("---------------------------------------");

        for line in passlines.iter() {
            if options.show_password {
                println!("{0}\t\t{1}", line.name, line.pass);
            } else {
                println!("{0}\t\t{1}", line.name, "*******");
            }
        }

        Ok(())
    }
}
