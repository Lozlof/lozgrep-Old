#[cfg(test)] // This attribute tells the Rust compiler to only compile this module when running tests. It is ignored during normal compilation to keep test code separate from production code.
mod tests { // Declares a module named tests. Modules in Rust help organize code, and test code is often put in a separate tests module.
    use std::fs;
    use crate::io;

    #[test] // This attribute marks the following function as a test. When cargo test is run, Rust will look for functions marked with #[test] and execute them as individual tests.
    fn one_result() {
        let query:String = format!("dreary");
        let contents_result: Result<String, io::Error> = fs::read_to_string("testfile.txt"); 

        let contents:String = match contents_result {
            Ok(file) => file,
            Err(error_one) => panic!("Problem reading the file contents of the given path: {}", error_one),
        };

        // assert_eq! is a macro that compares two values for equality. If the values are equal, the test passes and continues. If the values are not equal, the test fails, and Rust outputs an error showing the expected and actual values for easier debugging.
        // vec!["How dreary to be somebody!"] creates a vector containing a single string slice. This is the expected result for the test. The idea is that search should return this vector if it finds a line in contents that matches the search query. 
        // search(query, contents) calls the search function with query and contents as arguments.
        assert_eq!(vec!["How dreary to be somebody!"], search(&query, &contents));
    }

    pub fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
        vec![]
    }
}

