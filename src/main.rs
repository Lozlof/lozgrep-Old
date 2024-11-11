fn main() {
    lozgrep::create_log_file(); // Creates the log file and writes the first log entry.
    let parameters: lozgrep::QueryAndFileContent = lozgrep::build_arguments_and_collect_content(); // Validates the arguments, validates the path, validates the file content. Returns struct parameters.query_item, parameters.file_path, parameters.file_content, all are String.
    lozgrep::log_recieved_parameters(&parameters); // Writes a log entry saying that parameters have been recieved.  
}