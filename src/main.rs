use clap::Parser;
use rand::distributions::Uniform;
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::StdRng;

/// Generate & store your passwords safely
#[derive(Parser)]
struct Args {

    /// Name of the app this password is for
    #[clap(short, long, value_parser)]
    name: String,
}

fn main() {
    // let args = Args::parse();

    // println!("{}", args.name);

    println!("{}", gen_password(16));
}


fn gen_password(len: usize) -> String {
    let letters = get_letter_space();
    let distrib = Uniform::new(0, letters.len());
    
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    let mut rng = StdRng::from_seed(seed);
    
    let mut result = vec![];
    for _ in 0..len {
        result.push(letters[rng.sample(distrib)]);
    }

    result.into_iter().collect()
}


fn get_letter_space() -> Vec<char> {
    let mut letters: Vec<char> = vec![];

    for ch in 'a' as u8 .. 'z' as u8 + 1 {
        letters.push(ch as char);
    }

    for ch in 'A' as u8 .. 'Z' as u8 + 1 {
        letters.push(ch as char);
    }

    for n in '0' as u8 .. '9' as u8 + 1 {
        letters.push(n as char);
    }

    let mut special_chars = vec!['!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-'];
    letters.append(&mut special_chars);

    letters
}