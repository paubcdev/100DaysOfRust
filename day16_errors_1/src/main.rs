/* Handling errors in Rust
In order to handle errors in the code, Rust has a built-in macro called panic!
Panics, in general, can be caused two ways: unintentional by a library calling a panic due to a compile error or user-defined panics.

User-defined panics can be implemented when a function or part of the code expects something but, because of user action, it is not provided. They can also be used to signal
an error during the compile/execution of the code.
*/

fn main() {
    println!("{}", quarter_of(0));
}

// Kata example
fn quarter_of(month: u8) -> u8 {
    match month{
        1 | 2 | 3 => 1,
        4 | 5 | 6 => 2,
        7 | 8 | 9 => 3,
        10 | 11 | 12 => 4,
        _ => panic!("Input a valid month!") // In this case, we expect a value bewteen 1 and 12, if ANYTHING else is provided, it panics and displays a custom message.
    }
}

/* The provided example in the Rust Programming Language book of a library panic error is a perfect way of showing why a code might compile, but, due to being, in this case, insecure,
it panics and gives an error:

{
    let v = vec![1, 2, 3];

    v[99];
}

The code above is syntactically correct, it does not break any Rust language rules, but since it's trying to access a position that *does not exist* within the vector, it will
panic at compile time, telling that there's no element that can be accessed. The compiler will tell exactly the reason why and not execute the code after it.
 */