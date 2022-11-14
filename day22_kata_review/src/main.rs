use std::vec;

fn main() {
    println!("{}", is_square(26));
    let v1 = vec![2334454,5];
    println!("{:?}", min_max(&v1));
    let v2 = vec![1,2,2,2,3,3,3];
    let v3 = vec![3];
    println!("{:?}", array_diff(v2, v3))
}

// https://www.codewars.com/kata/54c27a33fb7da0db0100040e/rust
fn is_square(n: i64) -> bool {
    let x: f64 = n as f64;
    if x.sqrt().fract() == 0.0 {
        true
    } else {
        false
    }
}

// https://www.codewars.com/kata/559590633066759614000063/rust
fn min_max(lst: &[i32]) -> (i32, i32) {
    let min: &i32 = lst.iter().min().unwrap();
    let max: &i32 = lst.iter().max().unwrap();

    (*min, *max)

}

// https://www.codewars.com/kata/523f5d21c841566fde000009/rust
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}
