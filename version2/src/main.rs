use std::env;
use std::process;

fn main() {
    let possible_arguments: [&'static str; 4] = ["--help", "-h", "--path", "-p"]; // Array of all the possible arguments.
    let collected_arguments: Vec<String> = env::args().collect(); // Will collect passed arguments and put them into a vector.

    let total_num_collected_args: usize = collected_arguments.len(); // Gets the length of the collected arguments.

    if total_num_collected_args <= 1 { // If one or less arguments are passed, there is an issue.
        println!("Not enough arguments were passed");
        process::exit(1);
    }
    

}
