use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub struct SearchConfig {
    pub executable_path: String,
    pub query: String,
}

impl SearchConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<SearchConfig, Box<dyn Error>> {
        // Ignore binary command
        args.next();

        let executable_path = match args.next() {
            Some(val) => val.to_string(),
            None => return Err("Expected a binary executable program".into()),
        };

        let query = match args.next() {
            Some(val) => val.to_string(),
            None => return Err("Expected a string to search".into()),
        };

        Ok(SearchConfig {
            executable_path,
            query,
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

// Better approach: Separate IO and parsing
// fn find_offset_in_bytes(program_bytes: &[u8], query_text: &[u8]) -> Option<usize>
pub fn find_offset(program: &SearchConfig) -> Result<Option<usize>, Box<dyn Error>> {
    let program_bytes = parse_file(&program.executable_path)?;

    let query_text = program.query.as_bytes();
    let query_length = query_text.len();
    // println!("Byte length of search string: {}", query_length);

    // Idomatic way
    let offset = program_bytes
        .windows(query_length)
        .enumerate()
        .find(|(_, window)| *window == query_text)
        .map(|(index, _)| index);
    Ok(offset)

    // for index in 0..program_bytes.len() - query_length {
    //     let subset = &program_bytes[index..index + query_length];
    //     if subset == query_text {
    //         return Ok(Some(index));
    //     }
    // }
    // Ok(None)
}
