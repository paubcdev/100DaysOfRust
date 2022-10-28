/*
In order to return a value from a function, in the declaration process of said function,
the type of value that it's returned must be specified, with an arrow "->".
*/

// For instance, this function will return an integer (in this case a fixed value)
fn five() -> i32 {
    5
}

// In general, the compiler interprets the lack of a semicolon ";" as the return value
fn plus_one(x: i32) -> i32 {
    x + 1 // if we were to put a semicolon here, it'd throw an error
}

/*
Arrays are collections of multiple values. Different to tuples, arrays must only
contain elements that are the same type, no exceptions. Also, arrays in Rust
have a fixed length.
*/

/*
There's another way of declaring same-value arrays: you specify the value of the
array, then write a semicolon, then the size of the array (example arr_b).
*/

// To access an element, indexing (similar to Pyhton) is used.

/*
"if" expressions work similarly as in other languages, they don't require the use of
parenthesis to evaluate the condition, and they can have exceptions and nested 
expressions with "else" and "else if". "if" only checks for the condition if it has the
same data type, for example, it doesn't check if the variable exists or not, (because NaN
doesn't exist on Rust).
*/

fn main() {
    let a = five();
    let b = plus_one(7);
    println!("The value of the first function is: {a}");
    println!("The value of the second function is: {b}");

    let arr_a = [67, 76, 34, 99]; // Values are declared separated by commas in brackets
    let arr_b = [32; 4]; // This creates an array of len 4, all with a value of 32
    let first_element = arr_a[0];
    println!("The first element of the array is: {first_element}");
    if 3 < 4 {
        println!("This statement is true!");
    }
}
