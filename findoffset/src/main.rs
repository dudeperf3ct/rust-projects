// This project is inspired by systems programming series by Efron Amber Licht
// Ref: https://eblog.fly.dev/startingsystems1.html#3-peeking-into-the-black-box-what-is-a-program-anyways

// Problem statement: Given a hello world executable, find the hello world string.
// Input: a binary executable of program
// Output: Whether executable contains hello world.
//
// To run this program, run the following two commands
// cargo run --bin hello
// cargo run --bin findoffset  -- target/debug/hello "Hello, world!"

// Quirky:
// There's hello at 2204543 and Hello at 30896
// xxd target/debug/hello | less

use std::env;

use findoffset::{SearchConfig, find_offset};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program: SearchConfig = SearchConfig::build(env::args())?;
    println!(
        "Looking for {} in {} executable",
        program.query, program.executable_path
    );

    let offset = find_offset(&program)?;

    match offset {
        Some(value) => println!("Offset found at {}", value),
        None => println!("Offset not found!"),
    }

    Ok(())
}
