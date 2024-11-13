use std::env;
use std::process;

fn main() {
    let possible_arguments: [&'static str; 12] = ["--help", "-h", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf", "--verbose", "-v", "--version", "-ver"]; // Array of all the possible arguments.
    let collected_arguments: Vec<String> = env::args().skip(1).collect(); // Will collect passed arguments and put them into a vector.

    // let total_num_collected_args: usize = collected_arguments.len(); // Gets the length of the collected arguments.

    println!("{}", collected_arguments.len());

    if collected_arguments.len() < 1 { // If one or less arguments are passed, there is an issue.
        println!("Not enough arguments were passed");
        process::exit(1);
    }

    if collected_arguments.len() >= 20 { // If too many arguments are passed, there is an issue. 
        println!("Too many arguments were passed"); // TODO: Make this number persise.
        process::exit(1);
    }

    

    // let mut parse_arguments_counter: usize = 1;

    /* while parse_arguments_counter <= total_num_collected_args {

        parse_arguments_counter += 1;
    } */
}
