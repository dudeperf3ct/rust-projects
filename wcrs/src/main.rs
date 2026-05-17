// wc Tool - https://codingchallenges.fyi/challenges/challenge-wc
//
// ccwc -c filename -> Number of bytes
// ccwc -l filename -> Number of files
// ccwc -w filename -> Number of words
// ccwc -m filename -> Number of characters
// Example:
// > ccwc -c test.txt
//   342108 test.txt

// Version 1

// use std::{fs, io::Error, path::PathBuf};

// use clap::Parser;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about=None)]
// struct Cli {
//     // Name of file
//     filename: PathBuf,

//     // Count number of bytes
//     #[arg(short('c'), long)]
//     bytes: bool,

//     // Count number of lines
//     #[arg(short('l'), long)]
//     lines: bool,

//     // Count number of words
//     #[arg(short('w'), long)]
//     words: bool,

//     // Count number of characters
//     #[arg(short('m'), long)]
//     chars: bool,
// }

// fn parse_file(file_path: &PathBuf) -> Result<String, Box<Error>> {
//     let contents = fs::read_to_string(file_path)?;
//     Ok(contents)
// }

// fn main() {
//     let cli = Cli::parse();
//     let file_path = cli.filename;

//     let filename = file_path.file_name().unwrap();

//     let file_contents = parse_file(&file_path).unwrap();
//     let no_flags = !cli.bytes && !cli.lines && !cli.words && !cli.chars;

//     if cli.bytes || no_flags {
//         let bytes = file_contents.bytes().count();
//         print!("{:>8}", bytes);
//     }

//     if cli.lines || no_flags {
//         let lines = file_contents.lines().count();
//         print!("{:>8}", lines);
//     }

//     if cli.words || no_flags {
//         let words = file_contents.split_whitespace().count();
//         print!("{:>8}", words);
//     }

//     if cli.chars {
//         let chars = file_contents.chars().count();
//         print!("{:>8}", chars);
//     }

//     println!(" {}", filename.display())
// }

// Version 2
// Split the logic across library and keep lean main
// Learnings
// 1. Separating lib.rs -> Business logic 
// and main.rs -> calling library code, main entrypoint
// 2. Using clap for parsing in version 1 vs std lib for version 2
// 3. Good unit testing coverage

use std::{env, process::exit};

use ccwc::{CommandConfig, run};

fn main() {
    let command_config: CommandConfig = CommandConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error encountered in parsing the command: {err}");
        exit(1);
    });

    if let Err(e) = run(command_config) {
        eprintln!("Encounted error: {e}");
        exit(1);
    }
}
