use lozgrep::create_log_file;

fn main() {
    let create_log: Result<(), std::io::Error> = create_log_file();
}