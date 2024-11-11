fn main() {
    lozgrep::create_log_file();
    let config: lozgrep::Config = lozgrep::build_arguments_and_collect_content();
}