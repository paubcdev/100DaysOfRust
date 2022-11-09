/* Error handling, part 2.
Besides panic, there's other ways of handling errors in Rust. One of the more common ones is using a special type of "enum" called Option<T>; this way the program assumes that
it is possible to have an option that doesn't match the expected input and therefore an error occurs, but also that is recoverable.
The general usage is, using a match structure:
    - if an option with type "T" is found, (using some("T")) then the program resumes as normal;
    - if option with "T" is not found, (using none), the program continues, but not in the expected way.

In these cases, for example, a program expects to be given a specific data, but if another kind of data shows up, its not catastrophical and the program has a way of resuming.
*/

/* A "sub-type" (to call it some way) of Option, is Result. In this way, instead of resuming the program, it halts and an error (defined by the programmer) is shown.
Therefore Result<T, E> can have one of two outcomes:
    - Ok(T): An element of type "T" is found, and the execution resumes.
    - Err(E): An error is found (generally with element E) and execution is halted.

In these cases, if the data expected by the program is not given, the program has not a way of continuing, the program shows the error and stops.
*/


/* In this example of error handling, the program expects a file to open and then do something with the file. At compile time, the program doesn't know if such file exists or not in
the directory, so a Result is given: if the file exists (is found), then procede with the program, if it doesn't then an error is shown, and, in this case, it panics and halts execution
*/
use std::fs::File;
use std::io::Error;

fn main() {
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");

    let _greeting_file = match greeting_file_result { // Note that I'm using _var in order to not be shown a warning in the compiler
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
