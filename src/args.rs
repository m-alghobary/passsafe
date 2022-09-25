use clap::Parser;

/// Generate & store your passwords safely
#[derive(Parser, Debug, Default)]
pub struct Args {
    /// Name of the app this password is for
    #[clap(short, long, value_parser)]
    pub name: String,

    /// The length of the password
    #[clap(short, long, value_parser, default_value_t = 8)]
    pub len: usize,

    /// No uppercase letters
    #[clap(long, value_parser)]
    pub no_upper: bool,

    /// No lowercase letters
    #[clap(long, value_parser)]
    pub no_lower: bool,

    /// No numbers
    #[clap(long, value_parser)]
    pub no_numbers: bool,

    /// No special chartactors
    #[clap(long, value_parser)]
    pub no_special_chars: bool,
}
