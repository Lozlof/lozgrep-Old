use std::env; // To enable lozgrep to read the values of command line arguments we pass to it, we’ll need the std::env::args function provided in Rust’s standard library. 
pub mod logging;
pub use logging::*;

    pub fn collect_arguments() {
        let arguments: Vec<String> = env::args().collect(); // We are calling on the args function. And using the collect method.
        
        log_collected_arguments(&arguments);
    }
    
    /*// Declares the arguments variable with the Vec<String> type.
    // A vector is a growable array type, and this vector will hold strings.
    let arguments: Vec<String> = env::args().collect(); // We are calling on the args function. And using the collect method.
    debug_output_one(&arguments); // Pass arguments by reference means that main retains ownership of arguments.*/