![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

# Passgen
A command-line tool for generating customizable password. 

## Installation
### Requirements
- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on the system.


### Compilation
Clone the repo and compile the project with Cargo:
```bash
git clone https://github.com/naksudev/passgen.git
cd passgen
cargo build --release
```

## Usage
### Show help
```bash
passgen --help
```

### Generate a basic password
```bash
passgen -LNS --length 10 --exclude "aA"
```
This command generates a password **L**etters, **N**umbers and **S**pecial characters.  
It also excludes the letter A (upper and lower case) from the generation of the password.

### Available options
| Option              | Description                                          |
|---------------------|------------------------------------------------------|
| --length <LENGTH>   | Defines the length of the password (by default: 16). |
| -L                  | Include letters.                                     |
| -N                  | Include numbers.                                     |
| -S                  | Include special characters.                          |
| --exclude <EXCLUDE> | Exclude specific characters (by default: `none`).    |
| --help              | Show help                                            |
| --version           | Show version of the tool                             |


## Examples
### Password with 12 characters with letters and numbers only
```bash
passgen --length 12 -LN
```

### Exclude ambiguous characters (like `o`, `O`, `0`, `l`, `I`)
```bash
passgen --exclude "oOiI0lL"
```

### A very long and complex password.
```bash
passgen --length 64 -LNS
```

