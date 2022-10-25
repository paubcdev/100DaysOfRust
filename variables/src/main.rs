// Variables in Rust
// Use "let" to declare a variable, then "=" to assign it

fn main() {
    let x;
    x = 1;
    println!("{x}");

// It can also be single-line
    let y = 2;

// Variables' type can be explicitly specified, using ":"
    let z: i32;
    z = 3;

// Variables are immutable by default, to make them mutable, use "mut" when declaring the variable
    let mut a = 15;
    println!("First value is: {a}");
    a = 21;
    println!("Changed value now is: {a}");

// Now, both y and z variables are declared and not used, so the compiler is throwing a warning
// To declare variables and not use them, it is recommended to use an underscore _ before the variable name, so it won't throw a warning
    let _unvar;
    _unvar = 12;

// If a variable is declared but not initialized, the compiler will not allow its use, throwing an error
    let b;
    println!("{b}");
    b = 1;
}