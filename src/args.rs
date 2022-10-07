use clap::{Parser, Subcommand};

/// Generate & store your passwords safely
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
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
    },
}
