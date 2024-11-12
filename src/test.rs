#[cfg(test)] // This attribute tells the Rust compiler to only compile this module when running tests. It is ignored during normal compilation to keep test code separate from production code.
mod tests { // Declares a module named tests. Modules in Rust help organize code, and test code is often put in a separate tests module.
    use std::fs;
    use crate::io;

    #[test] // This attribute marks the following function as a test. When cargo test is run, Rust will look for functions marked with #[test] and execute them as individual tests.
    fn one_result() {
        let query: &str = "duct";
        let contents_result: Result<String, io::Error> = fs::read_to_string("testfile.txt"); 

        let contents:String = match contents_result {
            Ok(file) => file,
            Err(error_one) => panic!("Problem reading the file contents of the given path: {}", error_one),
        };
    }
}
