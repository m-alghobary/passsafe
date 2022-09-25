use crate::{passfile::PassFile, passgen::PassGenerator};
use args::Args;
use clap::Parser;
use passline::Passline;

mod args;
mod passfile;
mod passgen;
mod passline;

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
