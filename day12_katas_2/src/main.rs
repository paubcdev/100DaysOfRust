fn main() {
    println!("Kata 1: {}", past(1,30,0));
    println!("Kata 2: {}", opposite(-65));
    let sh_arr = [true, true, true, false, true, true, true, true, true, false, true, false, true, false, false, true, true, true,  true, true, false, false, true, true];
    println!("Kata 3: {}", count_sheep(&sh_arr));
    println!("Kata 4: {}", basic_op('+', 4, 7));
    println!("Kata 5: {}",enough(100, 60, 50));
    println!("Kata 5: {}", string_to_number("67"));
}

// Kata 1: given h, m & s, return the result in miliseconds.
fn past(h: i32, m: i32, s: i32) -> i32 {
    (h * 3600 + m * 60 + s) * 1000
}

// Kata 2: given an integer (or float), return its negative.
fn opposite(number: i32) -> i32 {
    -number
}

// Kata 3: function that counts the number of sheeps in an array.
fn count_sheep(sheep: &[bool]) -> u8 {
    let mut counter = 0;
    for n in sheep {
        if *n == true {
            counter+=1;
        }
    }
    counter
}

// Kata 4: given an operation, and two values, calculate the result.
fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    let mut result: i32 = 0;
    if operator == '+' {
        result = value1 + value2;
    } else if operator == '-' {
        result = value1 - value2;
    } else if operator == '*' {
        result = value1 * value2;
    } else if operator == '/' {
        result = value1 / value2;
    }
    result
}

// Kata 5: simple program that tells if it will be able to fit all the passengers in a bus (cap: amount of people; on: people already on the bus; wait: people waiting to get in the bus).
fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let realcap: i32 = cap - on;
    if (wait - realcap) <= 0 {
        0
    } else {
        (wait - realcap).abs()
    }
}

// Kata 6: transform a string into a number
fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}