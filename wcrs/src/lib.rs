use std::{error::Error, fs};

#[derive(Debug)]
pub enum Operation {
    Words,
    Lines,
    Bytes,
    Characters,
}

pub struct CommandConfig {
    pub operations: Vec<Operation>,
    pub file_paths: Vec<String>,
}

impl CommandConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<CommandConfig, &'static str> {
        let mut operations: Vec<Operation> = vec![];
        let mut file_paths: Vec<String> = vec![];
        args.next(); // skip the binary crate name

        for arg in args {
            if arg.starts_with("-") {
                match arg.as_str() {
                    "-w" => operations.push(Operation::Words),
                    "-c" => operations.push(Operation::Bytes),
                    "-l" => operations.push(Operation::Lines),
                    "-m" => operations.push(Operation::Characters),
                    _ => return Err("Invalid command"),
                }
            } else {
                file_paths.push(arg)
            }
        }

        // Default options
        if operations.is_empty() {
            operations.push(Operation::Lines);
            operations.push(Operation::Words);
            operations.push(Operation::Bytes);
        }

        if file_paths.is_empty() {
            return Err("Missing file path");
        }

        Ok(CommandConfig {
            operations,
            file_paths,
        })
    }
}

pub fn run(command_config: CommandConfig) -> Result<(), Box<dyn Error>> {
    // TODO: Handle input from stdin for empty path
    if command_config.file_paths.is_empty() {
        return Err("Missing file path".into());
    }

    for file_path in &command_config.file_paths {
        let contents = fs::read_to_string(file_path)?;
        for command in command_config.operations.iter() {
            let result = perform_operation(command, &contents);
            print!("{:>2}\t", result);
        }
        println!("{}", &file_path);
    }
    Ok(())
}

fn perform_operation(command: &Operation, contents: &str) -> usize {
    match command {
        Operation::Bytes => contents.len(),
        Operation::Characters => contents.chars().count(),
        Operation::Lines => contents.bytes().filter(|&b| b == b'\n').count(),
        Operation::Words => contents.split_whitespace().count(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_word_count() {
        let test_contents = "two words";
        let result = perform_operation(&Operation::Words, test_contents);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_lines_count() {
        let test_contents = "hello";
        let results = perform_operation(&Operation::Lines, test_contents);
        assert_eq!(results, 0)
    }

    #[test]
    fn test_bytes_count() {
        let test_contents = "hello";
        let results = perform_operation(&Operation::Bytes, test_contents);
        assert_eq!(results, 5)
    }

    #[test]
    fn test_chars_count() {
        let test_contents = "hello";
        let results = perform_operation(&Operation::Characters, test_contents);
        assert_eq!(results, 5)
    }

    #[test]
    fn test_empty_file_path() {
        let test_config = CommandConfig {
            operations: Vec::new(),
            file_paths: vec![],
        };
        let err = run(test_config).expect_err("expected missing file path error");
        assert_eq!(err.to_string(), "Missing file path");
    }

    #[test]
    fn test_invalid_command() {
        let test_command = "ccwc -x";
        let args = test_command
            .split_whitespace()
            .into_iter()
            .map(|x| x.to_string());
        let err = CommandConfig::build(args);
        assert_eq!(err.err(), Some("Invalid command"));
    }

    #[test]
    fn test_empty_command_uses_default_flags() {
        let args = vec!["ccwc".to_string(), "test.txt".to_string()];
        let cfg = CommandConfig::build(args.into_iter()).expect("config should parse");

        assert_eq!(cfg.file_paths, vec!["test.txt".to_string()]);
        assert_eq!(cfg.operations.len(), 3);
        assert!(matches!(cfg.operations[0], Operation::Lines));
        assert!(matches!(cfg.operations[1], Operation::Words));
        assert!(matches!(cfg.operations[2], Operation::Bytes));
    }

    #[test]
    fn test_missing_file_path() {
        let args = vec!["ccwc".to_string(), "-l".to_string()];
        let err = CommandConfig::build(args.into_iter());
        assert_eq!(err.err(), Some("Missing file path"));
    }
}
