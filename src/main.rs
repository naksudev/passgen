use clap::Parser;

mod cli;
use cli::Args;

mod password;
use password::PasswordConfig;

fn main() {
    let args = Args::parse();

    let config = PasswordConfig {
        length: args.length,
        letters: args.letters, 
        numbers: args.numbers,
        specials: args.specials,
        exclude: args.exclude,
    };

    let mut passwords = Vec::new();
    
    for _n in 1..args.count {
       passwords.push(password::generate_password(&config));
    }

    println!("Generated password(s):");
    for p in passwords {
        println!("{}", p);
    }
}
