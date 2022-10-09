pub fn want_to_re_generate() -> bool {
    println!("Re-Generate password [Y / N] ?");

    ask_yes_or_no()
}

pub fn want_to_update(name: &String) -> bool {
    println!("");
    println!("[{}] already exist!!", name);
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
