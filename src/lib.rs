use std::env; // To enable lozgrep to read the values of command line arguments we pass to it, we’ll need the std::env::args function provided in Rust’s standard library.
use std::process; // Used to exit.
pub mod logging;
pub use logging::*;

    pub fn collect_arguments() {
        let arguments: Vec<String> = env::args().collect(); // We are calling on the args function. And using the collect method. A vector is a growable array type, and this vector will hold strings.

        log_collected_arguments(&arguments); // Pass arguments by reference means that this function retains ownership of arguments.

        let configuration: Config = Config::build_arguments(&arguments).unwrap_or_else(|err: &str| { // Attempts to create a Config object by calling the associated function Config::build_arguments(&arguments).
            println!("{err}"); // Config::build_arguments(&arguments) returns a Result<Config, &'static str>, which is either Ok(config) if successful or Err(err) if an error occurs. .unwrap_or_else(|err| { ... }) is called on the Result to handle the two cases.
            process::exit(1); // If Ok(config), it unwraps and assigns it to config. If Err(err), it executes the closure, which prints the error message and exits the program with process::exit(1).
        }); // The variable config is an instance of the Config struct.

        log_built_config();
    }
    struct Config { // For full analysis on the ownership of query and file_path, see Rust-Loz/notes/minigrepnotes.md/Ownership analysis - struct Config.
        query: String,
        file_path: String,
    }   
    
    impl Config { // Starts an implementation block for the Config struct, where associated functions (methods) are defined.
        fn build_arguments(args: &[String]) -> Result<Config, &'static str> { // args: &[String] means it takes a reference to a slice of Strings (an array-like view into the vector). Returns a Result that is either Ok(Config) on success or Err(&'static str) on failure.
    
            if args.len() < 3 {
                return Err("Not enough arguments were passed.");
            }
            
            if args.len() > 3 {
                return Err("Too many arguments were passed.");
            }
    
            let query: String = args[1].clone(); // The clone method on the values. This will make a full copy of the data for the Config instance to own.
            let file_path: String = args[2].clone();
    
            return Ok(Config { query, file_path })
        }
    }