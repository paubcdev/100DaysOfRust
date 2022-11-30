// Filesystem manipulation and I/O, part 2.

use std::path::Path; // for part 2.1 and 2.2
use std::fs::File; // for part 2.2
use std::io::prelude::*; // for part 2.2
fn main() {
    path_example();
    create_example();
}

/* 2.1 As briefly touched on yesterday's code, the Path struct it's used to represent filepaths in the underlaying system. For Unix-like system "posix::Path" is used and
"windows::Path" is for Windows. Path structs are immutable. Path has a owned version called PathBuf. They are similar to String and str: PathBuf can be mutated in
place and can be dereferenced to a normal Path. To convert a Path to a usable string of characters, an Option must be used. */

fn path_example() {
    let path = Path::new("testfile.tgz");
    match path.to_str() { // This is because Paths are vectors of bytes, and thus, its conversion can result in an error.
        None => panic!("new path is not a valid sequence!"),
        Some(s) => println!("new path is {}", s),
    }
}

/* Create function. As the name implies, the "create" function is used to open a file in "write-only" mode. If a file with the same name as the introduced already existed,
its contents are destroyed and replaced by the ones passed in the program. 
In this example, a constant (static) is used to introduce some string into a textfile. Error handling is done by default. */
static TEXT: &str =
"This is just a really long text string that I am using to test this function of the Rust language. If all goes well, it should write it on a new text file and then tell us it did in the console.
Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

fn create_example() {
    let path = Path::new("example_text.txt");
    let display = path.display();

    // Open the file in write-only mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Insert the TEXT string to the file just created
    match file.write_all(TEXT.as_bytes()) {
        Err(why) => panic!("couldn't write {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}