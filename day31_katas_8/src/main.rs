use itertools::Itertools;

fn main() {
    println!("Kata 1: {}", litres(3.0));
    println!("Kata 2: {}", longest("thisis", "justatest"));
    println!("Kata 3: {}", two_sort(&["pau", "is", "tired"]));
}

// https://www.codewars.com/kata/582cb0224e56e068d800003c/rust
fn litres(time: f64) -> i32 {
    let x: i32 = time as i32 / 2;
    x
}

// https://www.codewars.com/kata/5656b6906de340bd1b0000ac/rust
fn longest(a1: &str, a2: &str) -> String {
    format!("{}{}", a1, a2).chars()
    .sorted()
    .dedup()
    .collect()
}

// https://www.codewars.com/kata/57cfdf34902f6ba3d300001e/rust
fn two_sort(arr: &[&str]) -> String {
    arr.iter().min().unwrap().chars().map(|c| c.to_string())
    .collect::<Vec<_>>().join("***")
}

// Unitary tests for algorithms
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn liters_test() {
        assert_eq!(litres(3.0), 1);
    }

    #[test]
    fn longest_test() {
        assert_eq!(longest("qwertyuiopasdfghjkl", "zxcvbnm"), "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn two_sort_test() {
        assert_eq!(two_sort(&["pau", "is", "tired", "aaa"]), "a***a***a")
    }
}