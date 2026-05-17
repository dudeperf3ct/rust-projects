// Version 1
// use std::{
//     env,
//     fs::File,
//     io::{self, Read},
// };

// #[derive(Debug)]
// struct GrepArgs {
//     search_string: String,
//     filename: String,
// }

// impl GrepArgs {
//     fn validate_search_string(&self) -> bool {
//         &self.search_string != ""
//     }

//     fn read_file(&self) -> Result<String, io::Error> {
//         let mut content = String::new();
//         File::open(&self.filename)?.read_to_string(&mut content)?;
//         Ok(content)
//     }

//     fn search(&self) -> Vec<String> {
//         if self.validate_search_string() {
//             let file_contents = self.read_file().expect("Failed to read input file");
//             file_contents
//                 .lines()
//                 .filter(|line| line.contains(&self.search_string))
//                 .map(|line| line.to_string())
//                 .collect()
//         } else {
//             let search_string = &self.search_string;
//             panic!("Invalid search string, got {search_string}")
//         }
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     dbg!(&args);
//     let grep_args = GrepArgs {
//         search_string: args[1].clone(), //String::from(&args[1]),
//         filename: args[2].clone(),      //String::from(&args[2]),
//     };
//     println!("Searching for in file {:#?}", grep_args);
//     let results = grep_args.search();
//     dbg!(&results);
// }

// Version 2 (from the rust book)
use minigrep::Config;
use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // Idomatic approaach for parsing arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
