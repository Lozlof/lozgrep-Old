use std::fs;
use std::fs::*;
use std::io::*;
use std::time::SystemTime;
use chrono::{DateTime, Local};

// Creates or overwrites the log file minigrep.log and writes the first entry to it.
/* pub fn create_log_file() -> Result<()> {
    get_current_time();

    let mut minigrep_log: File = fs::File::create("minigrep.log")?;
    
    minigrep_log.write_all(b"Create a new log file\n")?;
    
    Ok(())
}*/

pub fn create_log_file() {
    let minigrep_log_result: std::result::Result<File, Error> = fs::File::create("minigrep.log"); // Attempts to create a new file named "minigrep.log". If the file already exists, it truncates it (clears its contents). Returns a Result<File, Error> indicating success (Ok) or failure (Err).

    let mut minigrep_log: File = match minigrep_log_result { //  Matches on the Result to handle both success and error cases.
        Ok(file) => file, // If the file was created successfully, extract the File object. Assigns the File to minigrep_log. The File is mutable, allowing for read/write operations.
        Err(error) => panic!("Problem creating the minigrep.log file: {}", error) // If an error occurred during file creation, the program panics. Stops execution and prints the error message. Formats the error message with the error details. 
    };
}

fn get_current_time() {
    let current_time: SystemTime = SystemTime::now();
    let date_time: DateTime<Local> = current_time.into();
    let formatted_time: String = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Formatted Time: {}", formatted_time);
}