use colored::Colorize;
use rand::rngs::OsRng;
use rand::RngCore;
use std::collections::HashSet;

pub struct PasswordConfig {
    pub length: usize,
    pub letters: bool,
    pub numbers: bool,
    pub specials: bool,
    pub exclude: String,
}

impl PasswordConfig {
    fn validate(&self) {
        if !(self.letters || self.numbers || self.specials) {
            panic!("Error: At least one type of character must be enabled.");
        }

        if self.length == 0 {
            panic!("Error: Password length must be greater than 0.")
        }
    }

    fn build_charset(&self) -> HashSet<char> {
        let mut charset = HashSet::new();

        if self.letters {
            charset.extend("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
        }
        if self.numbers {
            charset.extend("0123456789".chars());
        }
        if self.specials {
            charset.extend("!@#$%^&*()-_=+[]{}|;:,.<>?/".chars());
        }

        for ch in self.exclude.chars() {
            charset.remove(&ch);
        }

        charset
    }
}

pub fn generate_password(config: &PasswordConfig) -> String {
    config.validate();
    let charset: Vec<char> = config.build_charset().into_iter().collect();

    let mut password = String::new();
    let mut rng = OsRng;

    for _ in 0..config.length {
        let index = rng.next_u32() as usize % charset.len();
        password.push(charset[index]);
    }

    password
}

pub fn colorize_password(password: &str) -> String {
    password
        .chars()
        .map(|c| {
            if c.is_digit(10) {
                c.to_string().bright_yellow().to_string()
            } else if c.is_alphanumeric() {
                c.to_string().blue().to_string()
            } else {
                c.to_string().bright_purple().to_string()
            }
        })
        .collect()
}
