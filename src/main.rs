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

fn re_generate() -> bool {
    println!("");
    println!("Re-Generate password [Y / N] ?");

    let mut answer = String::new();

    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input!");

    answer.trim().to_lowercase().eq("y")
}

// fn gen_password(len: usize) -> String {
//     let letters = get_letter_space();
//     let distrib = Uniform::new(0, letters.len());

//     let mut seed = [0u8; 32];
//     rand::thread_rng().fill_bytes(&mut seed);
//     let mut rng = StdRng::from_seed(seed);

//     let mut result = vec![];
//     for _ in 0..len {
//         result.push(letters[rng.sample(distrib)]);
//     }

//     result.into_iter().collect()
// }

// fn get_letter_space() -> Vec<char> {
//     let mut letters: Vec<char> = vec![];

//     for ch in 'a' as u8..'z' as u8 + 1 {
//         letters.push(ch as char);
//     }

//     for ch in 'A' as u8..'Z' as u8 + 1 {
//         letters.push(ch as char);
//     }

//     for n in '0' as u8..'9' as u8 + 1 {
//         letters.push(n as char);
//     }

//     let mut special_chars = vec![
//         '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-',
//     ];
//     letters.append(&mut special_chars);

//     letters
// }
