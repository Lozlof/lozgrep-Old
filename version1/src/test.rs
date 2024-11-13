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
    
    // Notice that we need to define an explicit lifetime 'a in the signature of search and use that lifetime with the contents argument and the return value.
    // In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
    // In other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument. This is important! The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly.
    pub fn search<'a>(search_query: &String, search_contents: &'a String) -> Vec<&'a str> {
        let mut search_results: Vec<&str> = Vec::new(); // We need a way to store the matching lines that we want to return. For that, we can make a mutable vector before the for loop and call the push method to store a line in the vector.
        
        for line in search_contents.lines() { // Iterate through each line of the contents.
            if line.contains(search_query) { // Check whether the line contains our query string.
                search_results.push(line); // Add found line to the list of values we’re returning.
            }
        }
        
        return search_results;
    }
}

