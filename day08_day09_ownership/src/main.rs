// This code does not work, it's just for illustrative purposes. Check the notes.md file in the parent directory in order to understand it.

fn main() {
// example 1, will get a compiler error, because the String type dies not have a way of copying without cloning
    let string1 = String::from("hello");
    let string2 = string1;

    println!("{}, world!", string1);
 
 // example 2, does not need to call a clone method to use 
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    
}
