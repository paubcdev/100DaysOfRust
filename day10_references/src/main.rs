fn main() {

    // Example 1
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Example 2
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s)

}

// Example 1
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Example 2
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}