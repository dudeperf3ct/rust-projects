// This project is inspired by systems programming series by Efron Amber Licht
// Ref: https://eblog.fly.dev/startingsystems1.html#4-6-reaching-into-files-with-torso-go

// Problem statement: Given a hello world executable and a string, read the bytes before and after the offset
// Input: a binary executable of program and a input string
// Output: Show the before N bytes and after N bytes from the offset
//
// To run this program, run the following commands
// cargo run --bin hello
// cargo run --bin torso -- --from target/debug/hello --query "Hello" --before 128 --after 128

use std::{
    error::Error,
    io::{self, Write},
};

use clap::Parser;
use torso::{Cli, SearchConfig, find_offset_in_bytes, parse_file, read_program_offset_range};

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = Cli::parse();
    let mut program = SearchConfig::build(&cli_args)?;

    let program_bytes = parse_file(&cli_args.executable_path)?;
    let offset = find_offset_in_bytes(&program_bytes, cli_args.query_text.as_bytes())
        .ok_or("offset not found")?;
    program.offsets.offset = offset;

    let (left_chunk, right_chunk) = read_program_offset_range(&program_bytes, &program.offsets)?;
    let mut out = io::stdout().lock();
    out.write_all(left_chunk)?;
    out.write_all(right_chunk)?;
    Ok(())
}
