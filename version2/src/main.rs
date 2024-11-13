use std::env;
use std::process;

fn main() {
    let possible_options: [&'static str; 12] = ["--help", "-h", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf", "--verbose", "-v", "--version", "-ver"]; // Array of all the possible options.
    let collected_options: Vec<String> = env::args().skip(1).collect(); // Will collect passed arguments and put them into a vector. All arguments are options. Does not collect the first passed argument, because it is not needed.

    // let total_num_collected_args: usize = collected_options.len(); // Gets the length of the collected options.

    println!("{}", collected_options.len());
    println!("{}", collected_options[0]);

    if collected_options.len() < 1 { // If one or less options are passed, there is an issue.
        println!("Not enough options were passed");
        process::exit(1);
    }

    if collected_options.len() >= 20 { // If too many options are passed, there is an issue. 
        println!("Too many options were passed"); // TODO: Make this number persise.
        process::exit(1);
    }

    if collected_options.len() == 1 { // The help option is the only singular option, therefore it is an error if there is only one collected option and it is not help.
        if collected_options[0] != "--help" && collected_options[0] != "-h" {
            println!("Invalid syntax"); // TODO: Make this error message more descriptive.
            process::exit(1);

        } else { // Help was passed, so the help message is printed. 
            print_help_message();
        }
    }

    

    // let mut parse_options_counter: usize = 1;

    /* while parse_options_counter <= total_num_collected_args {

        parse_options_counter += 1;
    } */
}

fn print_help_message() { // Prints the help message and exits.
    println!("Options:");
    println!("-h, --help");
    println!("-p, --path");
    process::exit(1);
}
