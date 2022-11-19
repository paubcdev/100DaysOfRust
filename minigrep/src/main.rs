// This command line program, following the instructions laid in the Rust-lang book, will become a clone, mini version, of the well known grep command in Unix-like systems.
// Program, in this iteration, takes two arguments: the string we want to find, and the path to the file we want to search into.

use std::env; // From the standard library of Rust, we will be using the env module, specifically the args function, which returns arguments passed via command line (in this case)
use std::fs; // and the fs module, whiocj handles the filesystem manipulation operations

fn main(){
    let args: Vec<String> = env::args().collect(); // by using .collect(), we're telling Rust to turn the iterator (values passed) into a vector, thus initializing the "args" variable
    // dbg!(args); | the debug function (also part of the standard library)

    let query = &args[1]; // since it's a vector, we can use indexing to take the value passed and initializing it into said vector
    let file_path = &args[2]; // same as before

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path) // the file is read and put into a string, using the read_to_string() function from the fs module
        .expect("Error: should have been able to read file"); // the function, besides the filepath, also takes an error argument, in order to display if a the give file doesn't exist. 
    
    println!("With text:\n{contents}");
}