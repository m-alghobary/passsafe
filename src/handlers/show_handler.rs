use crate::passfile::PassFile;

pub struct ShowOptions {
    pub name: String,
}

pub struct ShowHandler;

impl ShowHandler {
    pub fn handle(options: ShowOptions) -> std::io::Result<()> {
        let pass_file = PassFile::new();

        match pass_file.get_pass_line(&options.name)? {
            Some(pass_line) => {
                println!();
                println!("Name\t\tPassword");
                println!("---------------------------------------");
                println!("{0}\t\t{1}", pass_line.name, pass_line.pass);
            }
            None => eprintln!("No password with the given name!"),
        }

        Ok(())
    }
}
