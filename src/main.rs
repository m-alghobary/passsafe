use std::process;

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
    let pass_file = PassFile::new();

    let mut update_pass_line = false;
    if pass_file.pass_line_exist(&args.name)? {
        if !want_to_update(&args.name) {
            process::exit(0);
        }

        update_pass_line = true;
    }

    let gen = PassGenerator::new(&args);

    let mut password;
    loop {
        password = gen.gen_password();

        println!("");
        println!("Pass: {}", password);

        if !want_to_re_generate() {
            break;
        }
    }

    let pass_line = Passline::new(args.name, password);

    if !update_pass_line {
        pass_file.write_pass(&pass_line)?;
    } else {
        pass_file.update_pass(&pass_line)?;
    }

    pass_line.print();

    Ok(())
}

// Utils

fn want_to_re_generate() -> bool {
    println!("");
    println!("Re-Generate password [Y / N] ?");

    ask_yes_or_no()
}

fn want_to_update(name: &String) -> bool {
    println!("");
    println!("{} already exist!!", name);
    println!("Do you want to update it [Y / N] ?");

    ask_yes_or_no()
}

fn ask_yes_or_no() -> bool {
    let mut answer = String::new();

    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input!");

    answer.trim().to_lowercase().eq("y")
}
