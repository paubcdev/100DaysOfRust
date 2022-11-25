use itertools::Itertools;

fn main() {
    println!("Kata 1: {}", count_duplicates("lowercaseaaAb"));
    println!("Kata 2: {}", bouncing_ball(3.0, 0.66, 1.5));
    println!("Kata 3: {:?}", find_next_square(625));
}

// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/rust
fn count_duplicates(text: &str) -> u32 {
    text.to_lowercase().chars()
        .counts().values()
        .filter(|&&i| i > 1).count() as u32
}

// https://www.codewars.com/kata/5544c7a5cb454edb3c000047/rust
fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h <= 0.0 {
        panic!("Height must be higher than 0!")
    }
    if bounce <= 0.0 || bounce >= 1.0 {
        panic!("Bounce must be between 0 and 1!")
    }
    if window >= h {
        panic!("The window must be lower than the height!")
    }
    
    let mut num = 1;
    let mut boun_h = h * bounce;
    
    while boun_h > window {
        num += 2;
        boun_h *= bounce;
    }
    
    num as i32
}

// https://www.codewars.com/kata/56269eb78ad2e4ced1000013/rust
fn find_next_square(sq: u64) -> Option<u64> {
    let sq: f64 = (sq as f64).sqrt();
    if sq.fract() == 0.0 {
        Some((sq as u64 + 1).pow(2))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn duplicates_test() {
        assert_eq!(count_duplicates("lowercaseaaAb"), 2);
    }

    #[test]
    fn bouncing_test() {
        assert_eq!(bouncing_ball(3.0, 0.66, 3.1), 3);
    }

    #[test]
    fn next_square_test() {
        assert_eq!(find_next_square(625), Some(676));
    }
}