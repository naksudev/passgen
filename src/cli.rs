use clap::Parser;

#[derive(Parser)]
#[command(
    version, 
    name = "passgen",
    about = "Generate passwords with ease",
    long_about = "Passgen is a command-line tool for generating customizable password.\n\n\
        You can include letters, numbers, special characters and exclude ambiguous characters.",
    after_help = "Example:\n passgen -LNS --length 24 --exclude \"aAbBcC01\" --count 10"
)]

pub struct Args {
    /// Define the length of the password
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    /// Include letters (a-z, A-Z) in the password
    #[arg(short = 'L')]
    pub letters: bool,

    /// Include numbers (0-9) in the password
    #[arg(short = 'N')]
    pub numbers: bool,

    /// Include special characters (!@#$%^&*...)
    #[arg(short = 'S')]
    pub specials: bool,

    /// Exclude specific characters (e.g., 'aAbBcC01')
    #[arg(short, long, default_value_t = String::new())]
    pub exclude: String,

    /// Define the number of passwords to generate
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
}


