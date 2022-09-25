use crate::{passfile::PassFile, passgen::PassGenerator};
use clap::Parser;

mod passfile;
mod passgen;

/// Generate & store your passwords safely
#[derive(Parser, Debug, Default)]
pub struct Args {
    /// Name of the app this password is for
    #[clap(short, long, value_parser)]
    name: String,

    /// The length of the password
    #[clap(short, long, value_parser, default_value_t = 8)]
    len: usize,

    /// No uppercase letters
    #[clap(long, value_parser)]
    no_upper: bool,

    /// No lowercase letters
    #[clap(long, value_parser)]
    no_lower: bool,

    /// No numbers
    #[clap(long, value_parser)]
    no_numbers: bool,

    /// No special chartactors
    #[clap(long, value_parser)]
    no_special_chars: bool,
}

#[derive(Debug, Default)]
pub struct Passline {
    name: String,
    pass: String,
}

impl Passline {
    pub fn new(name: String, pass: String) -> Self {
        Self { name, pass }
    }

    pub fn print(&self) {
        println!("");
        println!("Name: {}", self.name);
        println!("Pass: {}", self.pass);
    }

    pub fn format(&self) -> String {
        String::from(format!("{0}: {1}\n", self.name, self.pass))
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let gen = PassGenerator::new(&args);
    let pass_file = PassFile::new();

    let mut password;
    loop {
        password = gen.gen_password();

        println!("");
        println!("Pass: {}", password);

        if !re_generate() {
            break;
        }
    }

    let pass_line = Passline::new(args.name, password);
    pass_file.write_pass(&pass_line)?;
    pass_line.print();

    Ok(())
}

// Utils

fn re_generate() -> bool {
    println!("");
    println!("Re-Generate password [Y / N] ?");

    let mut answer = String::new();

    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input!");

    answer.trim().to_lowercase().eq("y")
}
