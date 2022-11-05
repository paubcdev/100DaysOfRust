fn main() {
    println!("Kata 1: {}", paperwork(5, 0));
    println!("Kata 2: {}", get_volume_of_cuboid(3.0, 5.0, 6.1));
    println!("Kata 3: {}", dna_to_rna("GCAT"));
    println!("Kata 4: {}", positive_sum(&[]))
}

fn paperwork(n: i16, m: i16) -> u32 {
    if n < 0 {
        0
    } else if m < 0 {
        0
    } else {
        (n*m).try_into().unwrap()
    }
}

fn get_volume_of_cuboid(length: f32, width: f32, height: f32) -> f32 {
    length*width*height
}

fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

fn positive_sum(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    if slice.is_empty() {
        0
    } else {
        for i in slice {
            if i >= &0 {
                sum += i;
            } else {
                sum += 0;
            }
        }
        sum
    }
}