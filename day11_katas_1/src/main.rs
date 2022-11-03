fn main() {
    let array1 = [34, 15, 88, 2, -34];
    println!("{}", find_smallest_int(&array1));
    println!("{}", greet("Ryan", "Pau"));
    println!("{}", simple_multiplication(2));
    println!("{}", bool_to_word(true))
}

// Kata 1: given an array of ints, find the smallest one.
fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

// Kata 2: function that gives a personalized greeting. This function takes two parameters: name and owner. If name and owner are equal returns one thing, else, another.
fn greet(name: &str, owner: &str) -> String {
    if name == owner {
        format!("Hello, boss")
    } else {
        format!("Hello, guest")
    }
}

// Kata 3: if a number is even, multiply it by 8, otherwise multiply by 9.
fn simple_multiplication(number: u8) -> u8 {
    if number % 2 == 0 {
        number*8
    } else {
        number*9
    }
}

// Kata 4: method that takes a bool and returns a "Yes" string for true, or a "No" string for false.
fn bool_to_word(value: bool) -> &'static str {
    match value{
        true => "Yes",
        false => "No"
    }
}