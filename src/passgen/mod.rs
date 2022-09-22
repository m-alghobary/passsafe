use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

pub struct PassGenerator {
    len: usize
}


impl PassGenerator {
    
    pub fn new(len: usize) -> Self {
        Self { len }
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
    
        for ch in 'a' as u8..'z' as u8 + 1 {
            letters.push(ch as char);
        }
    
        for ch in 'A' as u8..'Z' as u8 + 1 {
            letters.push(ch as char);
        }
    
        for n in '0' as u8..'9' as u8 + 1 {
            letters.push(n as char);
        }
    
        let mut special_chars = vec![
            '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-',
        ];

        letters.append(&mut special_chars);
        letters
    }
}