use std::env; // To enable lozgrep to read the values of command line arguments we pass to it, we’ll need the std::env::args function provided in Rust’s standard library.
use std::process; // Used to exit.
use std::fs; // Need to handle files, read_to_string.
use std::io; // Need for get_contents error handling.
use std::path::Path;
pub mod logging;
pub use logging::*;
pub mod test;
struct Config { // For full analysis on the ownership of query and file_path, see Rust-Loz/notes/minigrepnotes.md/Ownership analysis - struct Config.
    query: String,
    file_path: String,
}
pub struct QueryAndFileContent {
    pub query_item: String,
    pub file_path: String,
    pub file_content: String,
}

pub fn build_arguments_and_collect_content() -> QueryAndFileContent {
    let arguments: Vec<String> = env::args().collect(); // We are calling on the args function. And using the collect method. A vector is a growable array type, and this vector will hold strings.

    {
        let concatenated_args: &String = &arguments.join(", "); // Format the string before it gets passed to write_to_log_file.
        let collected_args: String = format!("Collected arguments: {}", concatenated_args);
        write_to_log_file(&collected_args); // Pass the collected arguments to the logging function. Logs the collected arguments. 
    }

    let configuration: Config = Config::build_arguments(&arguments).unwrap_or_else(|err: &str| { // Attempts to create a Config object by calling the associated function Config::build_arguments(&arguments).
        println!("{err}"); // Config::build_arguments(&arguments) returns a Result<Config, &'static str>, which is either Ok(config) if successful or Err(err) if an error occurs. .unwrap_or_else(|err| { ... }) is called on the Result to handle the two cases.
        process::exit(1); // If Ok(config), it unwraps and assigns it to config. If Err(err), it executes the closure, which prints the error message and exits the program with process::exit(1).
    }); // The variable config is an instance of the Config struct.

    {
        let fmt_config_and_path: String = format!("Going to search for {} in file {}", &configuration.query, &configuration.file_path); // format! returns a String not a $String. Will fall out of scope once operation is complete.
        write_to_log_file(&fmt_config_and_path); // Logs the parsed and build arguments.
    }

    let file_contents: String = get_contents(&configuration).unwrap_or_else(|err: &str| {
        println!("{err}");
        process::exit(1);
    });

    {
        let file_contents: String = format!("Was able to retrieve the contents of: {}", &configuration.file_path);
        write_to_log_file(&file_contents); // Logs that the file contents were put into a string.
    }

    let query_item_and_file_content: QueryAndFileContent = QueryAndFileContent {
        query_item: configuration.query.clone(),
        file_path: configuration.file_path.clone(),
        file_content: file_contents.clone(),
    };

    return query_item_and_file_content;
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

        return Ok(Config { query, file_path });
    }
}

fn get_contents(config: &Config) -> Result<String, &'static str> {
    let check_path: bool = Path::new(&config.file_path).exists(); // Path::new(&config.file_path).exists() checks if the path is valid and returns a boolean value.
        
    if check_path == false { // TODO: Check and see what happens when you give a file path that the current user does not have access to.
        return Err("The path given does not exist");
    }

    {
        let validated_path: String = format!("Verified that: {} is a valid path", &config.file_path);
        write_to_log_file(&validated_path); // Logs that the path was validated.
    }

    let contents_result: Result<String, io::Error> = fs::read_to_string(&config.file_path); // fs::read_to_string takes the file_path, opens that file, and returns a value of type std::io::Result<String> that contains the file’s contents.

    let contents:String = match contents_result { // TODO: Learn how to handle errors without panic.
        Ok(file) => file,
        Err(error_one) => panic!("Problem reading the file contents of the given path: {}", error_one),
    };

    return Ok(contents);
}