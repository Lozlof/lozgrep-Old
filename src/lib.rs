use std::env; // To enable lozgrep to read the values of command line arguments we pass to it, we’ll need the std::env::args function provided in Rust’s standard library.
use std::process; // Used to exit.
use std::error::Error; // Used for the run function error handling.
pub mod logging;
pub use logging::*;

    pub fn collect_arguments() {
        let arguments: Vec<String> = env::args().collect(); // We are calling on the args function. And using the collect method. A vector is a growable array type, and this vector will hold strings.

        log_collected_arguments(&arguments); // Pass arguments by reference means that this function retains ownership of arguments.

        let config: Config = Config::build(&arguments).unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });
    }

    struct Config {
        query: String,
        file_path: String,
    }   
    
    impl Config {    
        fn build(args: &[String]) -> Result<Config, &'static str> { 
    
            if args.len() < 3 {
                return Err("Not enough arguments were passed.");
            }
            
            if args.len() > 3 {
                return Err("Too many arguments were passed.");
            }
    
            let query: String = args[1].clone();
            let file_path: String = args[2].clone();
    
            return Ok(Config { query, file_path })
        }
    }