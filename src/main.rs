use clap::Parser;
use crate::passgen::PassGenerator;

mod passgen;


/// Generate & store your passwords safely
#[derive(Parser, Debug, Default)]
struct Args {
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


fn main() {
    let args = Args::parse();
    let gen = PassGenerator::new(16);

    
    let mut password;
    loop {
        password = gen.gen_password();

        println!("");
        println!("Pass: {}", password);

        if !re_generate() {
            break;
        }
    }

    println!("");
    println!("Name: {}", args.name);
    println!("Pass: {}", password);
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
