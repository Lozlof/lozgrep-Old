fn main() {
    lozgrep::create_log_file();
    let parameters: lozgrep::QueryAndFileContent = lozgrep::build_arguments_and_collect_content();
    println!("Query for: {}", parameters.query_item);
    println!("File Content: {}", parameters.file_content);
}