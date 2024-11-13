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
    query_item: String,
    path_item: String,
}

impl Options {
    fn interpret_and_build_syntax(collected_arguments: Vec<String>) {
        let all_possible_options: [&str; 14] = ["--help", "-h", "--version", "-ver", "--verbose", "-v", "--query", "-q", "--path", "-p", "--simple-grep", "-sg", "--simple-find", "-sf"];
        
        verify_options_are_valid(&all_possible_options, &collected_arguments); // Seperates the options (arguments that start with -- or -) from the rest of the arguments. Then compares the options against all_possible_options to see if the given options are valid. If there are bad options, the process ends with an error message. If all the options are valid, it returns the valid options to interpret_and_build_syntax. 
    }
}

fn verify_options_are_valid(borrow_all_possible_options: &[&str; 14], borrow_collected_arguments: &Vec<String>) {
    let filtered_options: Vec<String> = borrow_collected_arguments // Parses through all the collected arguments and pulls out any options (-- -).
        .iter() // creates an iterator.
        .filter(|option| option.starts_with("--") || option.starts_with("-")) // .filter(...) is used to retain only items that satisfy a given condition. |option| is a closure (anonymous function) parameter representing each item passed from the iterator. Checks if the String starts with -- or -.
        .cloned() // Is used to convert &String (a reference) into an owned String. This is necessary because we want to create a new vector with owned String values, rather than references to the original vector’s items
        .collect(); //  Takes the filtered and cloned items from the iterator and collects them into a new Vec<String>. This newly created vector is then assigned to filtered_options.

    let bad_options: Vec<String> = filtered_options// Parses through all the filtered options and checks if they are actual valid options.
        .iter()
        .filter(|option| !borrow_all_possible_options.contains(&option.as_str())) // Filters by options that are not contained inside of borrow_all_possible_options.
        .cloned()
        .collect();

    if !bad_options.is_empty() { // If bad_options is not empty, then it means that bad option were passed.
        let print_bad_options: String = bad_options.join(", "); // Turns the values of &bad_options into a string so a clear error message can be printed.
        
        if bad_options.len() == 1 { // Different error messages depending on how many errors there are.
            println!("Invalid syntax. An unknown option was passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", &print_bad_options);
            process::exit(1);

        } else {
            println!("Invalid syntax. Unknown options were passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", &print_bad_options);
            process::exit(1);
        }
    }

    let filtered_arguments: Vec<String> = borrow_collected_arguments
    .iter()
    .filter(|argument| !argument.starts_with("--") && !argument.starts_with("-"))// Will filter out all other passed arguments that are not options (--, 1).
    .cloned()
    .collect();

    if filtered_arguments.len() != 2 {
        if filtered_arguments.len() > 2 { // If there are more than two non-option arguments, it is an issue because the only options that take arguments are query and path.
            let print_bad_arguments: String = filtered_arguments.join(", ");
            println!("Invalid syntax. Too many values were passed: {}. Use \"--help\" or \"-h\" to see options and syntax.", print_bad_arguments);

        } else if filtered_arguments.len() == 0 {
            let query_present: bool = filtered_options.contains(&"--query".to_string()) || filtered_options.contains(&"-q".to_string()); // query and path_present will == true if they contain query or path options. 
            let path_present: bool = filtered_options.contains(&"--path".to_string()) || filtered_options.contains(&"-p".to_string());

            if query_present && path_present{ // Since query and path require arguments, it is an error if there are no arguments, but query or path is present.
                println!("Invalid syntax. The query (--query, -q) and path (--path, -p) options require a non-option value to follow it. Use \"--help\" or \"-h\" to see options and syntax.");

            } else if query_present {
                println!("Invalid syntax. The query (--query, -q) option requires a non-option value to follow it. Use \"--help\" or \"-h\" to see options and syntax.");

            } else if path_present {
                println!("Invalid syntax. The path (--path, -p) option requires a non-option value to follow it. Use \"--help\" or \"-h\" to see options and syntax.");
            }
        }
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
