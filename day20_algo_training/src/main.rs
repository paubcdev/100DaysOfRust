fn main() {
    println!("{}", persistence(39));
    println!("{}", persistence(62));
    println!("{}", xo("xxoo"));
    println!("{}", xo("xooxo"));
    println!("{}", hero(3, 1));
    println!("{}", is_triangle(3, 3, 3))
}

// https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec/rust
fn persistence(num: u64) -> u64 {
    let mut i = 0;
    let mut num = num;
    while num >= 10 {
        i+=1;
        num = num.to_string().chars().map(|x| x as u64 - 48).product::<u64>();
        }
    i
}

// https://www.codewars.com/kata/55908aad6620c066bc00002a/rust
fn xo(string: &'static str) -> bool {
    string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count()
}

// https://www.codewars.com/kata/59ca8246d751df55cc00014c/rust
fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= dragons * 2
}

// https://www.codewars.com/kata/56606694ec01347ce800001b/rust
fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    a + b > c && a + c > b && b + c > a
}