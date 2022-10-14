use std::io;

use args::{Args, Commands};
use clap::Parser;
use handlers::{add_handler::*, list_handler::*, show_handler::*};
use passline::Passline;

mod args;
mod handlers;
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
        } => AddHandler::handle(AddOptions {
            name,
            len,
            no_upper,
            no_lower,
            no_numbers,
            no_special_chars,
        }),

        Commands::List { show_password } => ListHandler::handle(ListOptions { show_password }),

        Commands::Show { name } => ShowHandler::handle(ShowOptions { name }),
        Commands::Delete { name } => todo!(),
    }
}
