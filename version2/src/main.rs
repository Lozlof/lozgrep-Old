use std::env;
use std::process;

struct Options {
    help: usize,
    version: usize,
    verbose: usize,
    query: usize,
    path: usize,
    simple_grep: usize,
    simple_find: usize,
}

impl Options {
    fn interpret_and_build_syntax(collected_arguments: Vec<String>) {
        let all_possible_options: [&str; 14] = ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"];
    }
}

fn main() {
    let collected_arguments_main: Vec<String> = env::args().skip(1).collect(); // Will collect passed arguments and put them into a vector. Does not collect the first passed argument, because it is not needed.

    initial_verify_options(&collected_arguments_main); // Initial_verify_options checks if zero arguments were passed, and if too many arguments were passed, in either senario the process will exit.. 

    let running_configuration= Options::interpret_and_build_syntax(collected_arguments_main);

    // let total_num_collected_args: usize = collected_options.len(); // Gets the length of the collected options.

    // println!("{}", collected_options.len());
    // println!("{}", collected_options[0]);

    /* if collected_options.len() == 1 { // The help option is the only singular option, therefore it is an error if there is only one collected option and it is not help.
        if collected_options[0] != "--help" && collected_options[0] != "-h" {
            println!("Invalid syntax"); // TODO: Make this error message more descriptive.
            process::exit(1);

        } else { // Help was passed, so the help message is printed. 
            print_help_message();
        }
    }*/

    

    // let mut parse_options_counter: usize = 1;

    /* while parse_options_counter <= total_num_collected_args {

        parse_options_counter += 1;
    } */
}

fn initial_verify_options(collected_arguments_verify: &Vec<String>) {
    if collected_arguments_verify.len() == 0 { // If no arguments are passed, it is an error.
        println!("Invalid syntax. Zero arguments were passed. Use \"--help\" or \"-h\" to see options and syntax.");
        process::exit(1);
    }

    if collected_arguments_verify.len() > 20 { // If too many arguments are passed, it is an error.
        println!("Invalid syntax. Too many arguments were passed. Use \"--help\" or \"-h\" to see options and syntax."); // TODO: Make this number more specific to what the actual max is..
        process::exit(1);
    }
}

fn print_help_message() { // Prints the help message and exits.
    println!("Options:");
    println!("-h, --help");
    println!("-p, --path");

    process::exit(1);
}
