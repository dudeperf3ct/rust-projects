use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

#[derive(Debug)]
pub struct SearchConfig {
    pub executable_path: PathBuf,
    pub query_text: String,
    pub offsets: OffsetRange,
}

#[derive(Debug)]
pub struct OffsetRange {
    pub offset: usize,
    pub before_offset: usize,
    pub after_offset: usize,
}

#[derive(Parser)]
#[command(long_about=None)]
pub struct Cli {
    #[arg(long = "from")]
    pub executable_path: PathBuf,

    #[arg(long = "before")]
    pub before_offset: usize,

    #[arg(long = "after")]
    pub after_offset: usize,

    #[arg(long = "query")]
    pub query_text: String,
}

impl SearchConfig {
    pub fn build(cli_args: &Cli) -> Result<SearchConfig, Box<dyn Error>> {
        Ok(SearchConfig {
            executable_path: cli_args.executable_path.clone(),
            query_text: cli_args.query_text.clone(),
            offsets: OffsetRange {
                offset: 0, // fill later
                before_offset: cli_args.before_offset,
                after_offset: cli_args.after_offset,
            },
        })
    }
}

pub fn parse_file(file_path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut contents = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn find_offset_in_bytes(program_bytes: &[u8], query_text: &[u8]) -> Option<usize> {
    let query_length = query_text.len();

    // Idomatic way
    program_bytes
        .windows(query_length)
        .enumerate()
        .find(|(_, window)| *window == query_text)
        .map(|(index, _)| index)
}

#[allow(clippy::type_complexity)]
pub fn read_program_offset_range<'a>(
    program_bytes: &'a [u8],
    r: &OffsetRange,
) -> Result<(&'a [u8], &'a [u8]), Box<dyn Error>> {
    if r.before_offset > r.offset {
        return Err("before_offset cannot be greater than offset".into());
    }

    let after_end = r
        .offset
        .checked_add(r.after_offset)
        .ok_or("offset + after_offset overflowed")?;

    let left = program_bytes
        .get((r.offset - r.before_offset)..r.offset)
        .ok_or("left range is out of bounds")?;

    let right = program_bytes
        .get(r.offset..after_end)
        .ok_or("right range is out of bounds")?;

    Ok((left, right))
}
