extern crate rand;
use rand::prelude::*;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut new = Robot {
            name: String::with_capacity(5),
        };
        new.reset_name();
        println!("{}", new.name);
        new
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name.clear();
        let mut rng = thread_rng();
        for _ in 0..2 {
            let ch: char = rng.gen_range('A' as u8, 'Z' as u8 + 1) as u8 as char;
            self.name.push(ch);
        }
        for _ in 0..3 {
            let num: char = rng.gen_range('0' as u8, '9' as u8 + 1) as u8 as char;
            self.name.push(num);
        }
    }
}
