use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

pub struct PassGenerator {
    len: usize,
    use_upper: bool,
    use_lower: bool,
    use_numbers: bool,
    use_special_chars: bool,
}

impl PassGenerator {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            use_upper: true,
            use_lower: true,
            use_numbers: true,
            use_special_chars: true,
        }
    }

    pub fn gen_password(&self) -> String {
        let letters = self.get_letter_space();
        let distrib = Uniform::new(0, letters.len());

        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        let mut rng = StdRng::from_seed(seed);

        let mut result = vec![];
        for _ in 0..self.len {
            result.push(letters[rng.sample(distrib)]);
        }

        result.into_iter().collect()
    }

    fn get_letter_space(&self) -> Vec<char> {
        let mut letters: Vec<char> = vec![];

        if self.use_lower {
            for ch in 'a' as u8..'z' as u8 + 1 {
                letters.push(ch as char);
            }
        }

        if self.use_upper {
            for ch in 'A' as u8..'Z' as u8 + 1 {
                letters.push(ch as char);
            }
        }

        if self.use_numbers {
            for n in '0' as u8..'9' as u8 + 1 {
                letters.push(n as char);
            }
        }

        if self.use_special_chars {
            let mut special_chars = vec![
                '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-',
            ];

            letters.append(&mut special_chars);
        }

        letters
    }
}
