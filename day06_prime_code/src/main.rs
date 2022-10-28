// Program to tell if a number is prime

fn main() {
    let x = 4; // Work on implementing user input in future iterations of this code
    let result = find_primes(x);
    println!("Is the number {x} prime?");
    if result == true {
        println!("Yes!");
    } else {
        println!("No!");
    }
}

fn find_primes(n: i32) -> bool { // Function takes an integer and returns a bool value
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

/*
Decided to make this because it's a good review of this week's learnt concepts:
return functions, data types and introduction to loops and comparators.
*/