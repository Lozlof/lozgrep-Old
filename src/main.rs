fn main() {
    lozgrep::create_log_file();
    let config: lozgrep::Config = lozgrep::collect_and_build_arguments();
}