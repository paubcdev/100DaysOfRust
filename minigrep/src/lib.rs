use std::error::Error; // This module form the standard collection allows to fine-tune error handling.
use std::fs; // and this fs module, which handles the filesystem manipulation operations

// We're going to create a configuration structure to group the values of the config part in the parse function.
pub struct Config {
    pub query: String,
    pub file_path: String,
    // this way it shows that query and file_path are related to the program and their purpose is to configure how the program works
}

/* Having main.rs handle both the reading and parsing of the progam is not ideal, since, if any error would rise, it would become really difficult to debug and fix.
We'll create a new function that is going to handle the logic of the parsing section of the code. This will migrate in the future to a different file.
Since the function now depends on the Config structure, we can make a method out of it.
By using a method now, it becomes clear what the function does and what it depends on.
*/

impl Config  {
    pub fn build(args: &[String]) -> Result<Config, &'static str> { // the input is a string vector and the output is going to be the config structure. Error handling by means of Result.
        if args.len() < 3 {
            return Err("not enough arguments"); // checks for the length of the argument vector: if fewer than the required 2 are passed, it throws an error.
        }

        let query = args[1].clone(); // clone in this case is used to fix ownership issues.
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
        // this is just the previous snippet of code, but refactored in a standalone function, takes the query and the file path and returns it for later use.
    }
}

// The logic is extracted from the main function and put into a separate run function, for readibility and modularity
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // the file is read and put into a string, using the read_to_string() function from the fs module

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}