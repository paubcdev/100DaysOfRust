// This command line program, following the instructions laid in the Rust-lang book, will become a clone, mini version, of the well known grep command in Unix-like systems.
// Program, in this iteration, takes two arguments: the string we want to find, and the path to the file we want to search into.

use std::env; // From the standard library of Rust, we will be using the env module, specifically the args function, which returns arguments passed via command line (in this case)
use std::process;

use minigrep::Config;

fn main(){
    let args: Vec<String> = env::args().collect(); // by using .collect(), we're telling Rust to turn the iterator (values passed) into a vector, thus initializing the "args" variable
    // dbg!(args); | the debug function (also part of the standard library)

    // This checks for errors that may appear in the build function, if they're found, it displays them, then stops.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    // This checks for any errors that may arise in the run function, displays them and then halts the execution. Similar to the method before.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}