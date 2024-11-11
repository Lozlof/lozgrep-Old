use std::fs;
use std::fs::*;
use std::io::*;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use std::fs::OpenOptions;
use std::io::Write;

pub fn create_log_file() {
    let lozgrep_log_result: std::result::Result<File, Error> = fs::File::create("lozgrep.log"); // Attempts to create a new file named "minigrep.log". If the file already exists, it truncates it (clears its contents). Returns a Result<File, Error> indicating success (Ok) or failure (Err).

    let mut lozgrep_log: File = match lozgrep_log_result { //  Matches on the Result to handle both success and error cases.
        Ok(file) => file, // If the file was created successfully, extract the File object. Assigns the File to minigrep_log. The File is mutable, allowing for read/write operations.
        Err(error_one) => panic!("Problem creating the lozgrep.log file: {}", error_one), // If an error occurred during file creation, the program panics. Stops execution and prints the error message. Formats the error message with the error details. 
    };

    let current_time: String = get_current_time();
    let log_entry: String = format!("{}: Created this log file\n", current_time); // Have to format the log entry before hand because .write_all does not do formatting.

    match lozgrep_log.write_all(log_entry.as_bytes()) { // Attempts to write data to a file. write_all is a method from the Write trait, which writes the entire byte slice passed to it. log_entry.as_bytes() converts the log_entry string into a byte slice (&[u8]) since write_all expects data in byte format.
        Ok(_) => {}, //  The write_all operation succeeds, Ok(_) matches the result. Here, _ is a placeholder, indicating that we don’t need to use the value Ok returns (which is () in this case). The {} is an empty block, meaning no additional action is taken if writing to the file succeeds.
        Err(error_two) => panic!("Problem writing to the lozgrep.log file: {}", error_two),
    }
}

pub fn log_collected_arguments(vec_arguments: &Vec<String>) { // log_collected_arguments &arguments meaning it does not take ownership of arguments. log_collected_arguments places the value of &arguments into the variable vec_arguments. 
    let lozgrep_log_result: std::result::Result<File, Error> = OpenOptions::new()// Opens lozgrep.log with write and append options, ensuring the file is not created if it doesn’t exist.
        .write(true)
        .append(true)
        .open ("lozgrep.log");
    
    let mut lozgrep_log: File = match lozgrep_log_result {
        Ok(file) => file,
        Err(error_one) => panic!("Problem opening the lozgrep.log file: {}", error_one),
    };

    let current_time: String = get_current_time();
    let concatenated_args: String = vec_arguments.join(", ");
    let log_entry: String = format!("{}: Collected arguments: {}\n", current_time, concatenated_args);

    match lozgrep_log.write_all(log_entry.as_bytes()) { 
        Ok(_) => {},
        Err(error_two) => panic!("Problem writing to the lozgrep.log file: {}", error_two),
    }
}

fn get_current_time() -> String { // TODO: The time is off by 5 hours.
    let current_time: SystemTime = SystemTime::now(); // Retrieves the current system time (SystemTime) from the operating system.
    let date_time: DateTime<Local> = current_time.into(); // The SystemTime is converted into a DateTime<Local>, which is a more convenient type from the chrono crate for working with dates and times in the local time zone.
    let formatted_time: String = date_time.format("%Y-%m-%d %H:%M:%S").to_string(); // Formats the date_time using the format "%Y-%m-%d %H:%M:%S". This formatted date and time are converted to a String.

    return formatted_time;
}