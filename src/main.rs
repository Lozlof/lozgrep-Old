fn main() {
    lozgrep::create_log_file(); // Creates the log file and writes the first log entry.
    let parameters: lozgrep::QueryAndFileContent = lozgrep::build_arguments_and_collect_content(); // Validates the arguments, validates the path, validates the file content. Returns struct parameters.query_item, parameters.file_path, parameters.file_content, all are String.
    {
        let recieved_parameters: String = format!("Main function recieved query parameters. Query for: {} in file {}. File content not written to log to reduce clutter.\n", &parameters.query_item, &parameters.file_path);
        lozgrep::write_to_log_file(&recieved_parameters); // Writes a log entry saying that parameters have been recieved.
    }
    let found_lines: Vec<&str> = simple_search(&parameters.query_item, &parameters.file_content);  

    for item in &found_lines {
        println!("{}", item);
    }
}

fn simple_search<'a>(search_query: &String, search_contents: &'a String) -> Vec<&'a str> {
    let mut search_results: Vec<&str> = Vec::new();
    
    for line in search_contents.lines() { 
        if line.contains(search_query) { 
            search_results.push(line);
        }
    }
    
    return search_results;
}