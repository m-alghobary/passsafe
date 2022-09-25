#[derive(Debug, Default)]
pub struct Passline {
    pub name: String,
    pub pass: String,
}

impl Passline {
    pub fn new(name: String, pass: String) -> Self {
        Self { name, pass }
    }

    pub fn print(&self) {
        println!("");
        println!("Name: {}", self.name);
        println!("Pass: {}", self.pass);
    }

    pub fn format(&self) -> String {
        String::from(format!("{0}: {1}\n", self.name, self.pass))
    }
}
