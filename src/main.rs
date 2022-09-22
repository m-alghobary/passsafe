use clap::Parser;
use crate::passgen::PassGenerator;

mod passgen;


/// Generate & store your passwords safely
#[derive(Parser)]
struct Args {
    /// Name of the app this password is for
    #[clap(short, long, value_parser)]
    name: String,
}


fn main() {
    let args = Args::parse();
    let mut password;
    let gen = PassGenerator::new(16);
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
