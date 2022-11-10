use itertools::Itertools; // must be declared in Cargo.toml file
fn main() {
    println!("{}", make_upper_case("test"));
}

// https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa
fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter().map(|(a, h)| {
        if a >= 55 && h > 7 {
            "Senior"
        } else {
            "Open"
        }
        .to_string()
    }).collect()
}

// https://www.codewars.com/kata/5a00e05cc374cb34d100000d
fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}

fn reverse_seq(n: u32) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new(); 
    let mut i = n;
    while i >= 1 {
        ret.push(i);
        i-=1;
    }
    ret
}

// https://www.codewars.com/kata/55ecd718f46fba02e5000029
fn between(a: i16, b: i16) -> Vec<i16> {
    let mut ret: Vec<i16> = Vec::new();
    let mut i = a;
    while i <= b {
        ret.push(i);
        i+=1;
    }
    ret
}

// https://www.codewars.com/kata/563cf89eb4747c5fb100001b
fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut numbers = numbers.to_vec();
    match numbers.iter().position_min() {
        None => numbers,
        Some(m) => {numbers.remove(m); numbers}
    }
}

// https://www.codewars.com/kata/5266876b8f4bf2da9b000362
fn likes(names: &[&str]) -> String {
    let leng = names.len();
    match leng {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        x => format!("{}, {} and {} others like this", names[0], names[1], x - 2),
    }
}