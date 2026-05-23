use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub struct SearchConfig {
    pub executable_path: String,
    pub query_text: String,
    pub replace_text: String,
}

impl SearchConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<SearchConfig, Box<dyn Error>> {
        // Ignore binary command
        args.next();

        let executable_path = match args.next() {
            Some(val) => val.to_string(),
            None => return Err("Expected a binary executable program".into()),
        };

        let query_text = match args.next() {
            Some(val) => val.to_string(),
            None => return Err("Expected a string to search".into()),
        };

        let replace_text = match args.next() {
            Some(val) => val.to_string(),
            None => return Err("Expected a string to search".into()),
        };

        if query_text.len() != replace_text.len() {
            return Err("The length of query text should be same replace text".into());
        }

        Ok(SearchConfig {
            executable_path,
            query_text,
            replace_text,
        })
    }
}

pub fn parse_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut contents = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut contents)?;
    // println!("Byte length of executable: {}", contents.len());
    Ok(contents)
}

pub fn find_offset_in_bytes(program_bytes: &[u8], query_text: &[u8]) -> Option<usize> {
    let query_length = query_text.len();
    // println!("Byte length of search string: {}", query_length);

    // Idomatic way
    program_bytes
        .windows(query_length)
        .enumerate()
        .find(|(_, window)| *window == query_text)
        .map(|(index, _)| index)

    // for index in 0..program_bytes.len() - query_length {
    //     let subset = &program_bytes[index..index + query_length];
    //     if subset == query_text {
    //         return Ok(Some(index));
    //     }
    // }
    // Ok(None)
}

pub fn replace_offset_with_string<'a>(
    program_bytes: &'a mut [u8],
    offset: usize,
    replace_text: &[u8],
) -> &'a [u8] {
    program_bytes[offset..offset + replace_text.len()].copy_from_slice(replace_text);
    program_bytes
}
