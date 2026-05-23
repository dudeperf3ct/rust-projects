// This project is inspired by systems programming series by Efron Amber Licht
// Ref: https://eblog.fly.dev/startingsystems1.html#4-5-basic-hacking-w-binpatch-go

// Problem statement: Given a hello world executable and a string, write these string inside the executable
// Input: a binary executable of program and a input string
// Output: Patch program by replacing the "Hello, world!" string with input string
//
// To run this program, run the following commands
// cargo run --bin hello
// cargo run --bin binpatch  -- target/debug/hello "Hello" "world"
// chmod +x target/debug/hello.patched
// ./hello.patched
// This outputs "world, world!"

use std::env;

use binpatch::{SearchConfig, replace_offset_with_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program: SearchConfig = SearchConfig::build(env::args())?;
    println!(
        "Looking for {} in {} executable and replace with {}",
        program.query_text, program.executable_path, program.replace_text
    );

    let mut program_bytes = binpatch::parse_file(&program.executable_path)?;
    let offset =
        binpatch::find_offset_in_bytes(program_bytes.as_slice(), program.query_text.as_bytes());

    match offset {
        Some(value) => println!("Offset found at {}", value),
        None => println!("Offset not found!"),
    }

    let new_program = replace_offset_with_string(
        program_bytes.as_mut_slice(),
        offset.unwrap(),
        program.replace_text.as_bytes(),
    );

    let out_path = "target/debug/hello.patched";
    println!("New binary program saved at {out_path}");
    std::fs::write(out_path, new_program)?;
    Ok(())
}
