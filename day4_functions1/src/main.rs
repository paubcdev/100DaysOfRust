/* Functions in Rust: to declare a function, the language uses the "fn" keyword;
functions should be named with lowercase words and use underscores to separate
multiple words (snake case convention). */

// the main() function its the entry point for most (if not all) Rust programs
fn main() { 
    println!("Hello, world!");
    new_function(5);
    print_degrees(32, 'C');
}

/* As with multiple progamming language, Rust doesn't care about when and where the
new functions are declared in the scope of the whole program, either before or after
the main() function. */

/* When declaring a function, Rust accepts parameters/arguments as part of the
declaration, so the function can use those arguments in its execution. When declared,
the argument must also contain its data type */

fn new_function(x: i32) {
    println!("The value of x is {x}");
}
// and the argument is passed when the funcion is called (refer to main function)

/* If multiple arguments are passed, they can be of different data types, and they
all must be explicitly declared. */
fn print_degrees(deg: i32, unit: char) {
    println!("The temperature is {deg} degrees {unit}.");
}