use crate::passfile::PassFile;

pub struct DeleteOptions {
    pub name: String,
}

pub struct DeleteHandler;

impl DeleteHandler {
    pub fn handle(options: &DeleteOptions) -> std::io::Result<()> {
        let pass_file = PassFile::new();

        if pass_file.delete_pass_line(&options.name)? {
            println!();
            println!("Password for [{0}] was successfully deleted", options.name);
        } else {
            eprintln!("No password with the given name!");
        }

        Ok(())
    }
}
