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

    let password = password::generate_password(&config);
    println!("Generated password: \n{}", password);
}
