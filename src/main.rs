use clap::Parser;


/// Generate & store your passwords safely
#[derive(Parser)]
struct Args {

    /// Name of the app this password is for
    #[clap(short, long, value_parser)]
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.name);
}