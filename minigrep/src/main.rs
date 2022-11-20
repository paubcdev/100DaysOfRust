// This command line program, following the instructions laid in the Rust-lang book, will become a clone, mini version, of the well known grep command in Unix-like systems.
// Program, in this iteration, takes two arguments: the string we want to find, and the path to the file we want to search into.

use std::env; // From the standard library of Rust, we will be using the env module, specifically the args function, which returns arguments passed via command line (in this case)
use std::fs; // and the fs module, whiocj handles the filesystem manipulation operations
use std::process;

fn main(){
    let args: Vec<String> = env::args().collect(); // by using .collect(), we're telling Rust to turn the iterator (values passed) into a vector, thus initializing the "args" variable
    // dbg!(args); | the debug function (also part of the standard library)

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path) // the file is read and put into a string, using the read_to_string() function from the fs module
        .expect("Error: should have been able to read file"); // the function, besides the filepath, also takes an error argument, to display if something unexpected happens. 
    
    println!("With text:\n{contents}");
}
// We're going to create a configuration structure to group the values of the config part in the parse function.
struct Config {
    query: String,
    file_path: String,
    // this way it shows that query and file_path are related to the program and their purpose is to configure how the program works
}

/* Having main.rs handle both the reading and parsing of the progam is not ideal, since, if any error would rise, it would become really difficult to debug and fix.
We'll create a new function that is going to handle the logic of the parsing section of the code. This will migrate in the future to a different file.
Since the function now depends on the Config structure, we can make a method out of it.
By using a method now, it becomes clear what the function does and what it depends on.
*/

impl Config  {
    fn build(args: &[String]) -> Result<Config, &'static str> { // the input is a string vector and the output is going to be the config structure. Error handling by means of Result.
        if args.len() < 3 {
            return Err("not enough arguments"); // checks for the length of the argument vector: if fewer than the required 2 are passed, it throws an error.
        }

        let query = args[1].clone(); // clone in this case is used to fix ownership issues.
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
        // this is just the previous snippet of code, but refactored in a standalone function, takes the query and the file path and returns it for later use.
    }
}