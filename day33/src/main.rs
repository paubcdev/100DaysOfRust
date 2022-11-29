/* The I/O - fs (file system) within the standard library is used to manipulate things within the filesystem, most commonly, opening and closing files, copying the contents,
or manipulating the files to which it has permission to operate on (renaming, deleting, etc). */

use std::fs::File;
use std::io::prelude::*;
use std::path::Path; // mainly for best practices

// The open function. This main function in the program will be used to open and display the contents of a simple text file. It includes error handling capabilities.
// (It assumes a file named "hello.txt" exists or it doesn't.)
fn main() {
    // First a path for the desired file is created in a variable.
    let path = Path::new("hello.txt");
    let display = path.display();

    // Then the set path is opened, in read mode only.
    let mut file = match File::open(&path) { // checks if the file is opened and
        Err(why) => panic!("couldn't open {}: {}", display, why), // if it doesn't tells why.
        Ok(file) => file,
    };

    // A string for the contents is created.
    let mut s = String::new();
    // Then it puts the contents of the file in said string.
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),        
    }
}