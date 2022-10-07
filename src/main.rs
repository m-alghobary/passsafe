use std::{io, process};

use crate::{passfile::PassFile, passgen::PassGenerator};
use args::{Args, Commands};
use clap::Parser;
use passline::Passline;

mod args;
mod passfile;
mod passgen;
mod passline;
mod utils;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(command) => {
            handle_command(command)?;
        }
        None => {
            eprintln!("Please supply a valid command!");
        }
    }

    Ok(())
}

fn handle_command(command: Commands) -> io::Result<()> {
    match command {
        Commands::Add {
            name,
            len,
            no_upper,
            no_lower,
            no_numbers,
            no_special_chars,
        } => {
            let pass_file = PassFile::new();

            let update_pass_line = is_update(&pass_file, &name)?;

            let mut gen = PassGenerator::new(len);
            gen.use_lower(!no_lower)
                .use_upper(!no_upper)
                .use_numbers(!no_numbers)
                .use_special_chars(!no_special_chars);

            let mut password;
            loop {
                password = gen.gen_password();

                println!("");
                println!("Pass: {}", password);

                if !utils::want_to_re_generate() {
                    break;
                }
            }

            let pass_line = Passline::new(name, password);

            if !update_pass_line {
                pass_file.write_pass(&pass_line)?;
            } else {
                pass_file.update_pass(&pass_line)?;
            }

            pass_line.print();
        }
    }

    Ok(())
}

fn is_update(pass_file: &PassFile, name: &String) -> io::Result<bool> {
    let mut update_pass_line = false;

    if pass_file.pass_line_exist(name)? {
        if !utils::want_to_update(name) {
            process::exit(0);
        }

        update_pass_line = true;
    }

    Ok(update_pass_line)
}
