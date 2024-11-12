fn main() { // TODO: Figure out thye logging situation.
    lozgrep::create_log_file(); // Creates the log file and writes the first log entry.

    let parameters: lozgrep::QueryAndFileContent = lozgrep::build_arguments_and_collect_content(); // Validates the arguments, validates the path, validates the file content. Returns struct parameters.query_item, parameters.file_path, parameters.file_content, all are String.
    
    {
        let recieved_parameters: String = format!("Main function recieved query parameters. Query for: {} in file {}. File content not written to log to reduce clutter", &parameters.query_item, &parameters.file_path);
        lozgrep::write_to_log_file(&recieved_parameters); // Writes a log entry saying that parameters have been recieved.
    }

    simple_search(&parameters.query_item, &parameters.file_content); // simple_search finds lines in the given file, and logs the status, then calls on simple_print_and_exit which will print the lines out and exit the program. 
}

fn simple_search<'a>(search_query: &String, search_contents: &'a String) {
    let mut search_results: Vec<&str> = Vec::new();
    
    for line in search_contents.lines() { 
        if line.contains(search_query) { 
            search_results.push(line);
        }
    }

    if !&search_results.is_empty() { // If &search_results in not empty.
        let search_results_status: String = format!("lozgrep found lines in the given file that matched the query parameters. Found items not written to log to reduce clutter");
        lozgrep::write_to_log_file(&search_results_status); // Writes a log entry saying that lozgrep found a match.

    } else {
        let search_results_status: String = format!("lozgrep did not find any lines in the given file that matched the query parameters");
        lozgrep::write_to_log_file(&search_results_status); // Writes a log entry saying that lozgrep did not find a match.
    }

    simple_print_and_exit(&search_results);
}

fn simple_print_and_exit(found_lines: &Vec<&str>) {
    
    for item in found_lines {
        println!("{}", item);
    }

    std::process::exit(1); // End of process.
}