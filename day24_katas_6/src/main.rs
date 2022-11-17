fn main() {
    println!("{}", divisors(4096));
    println!("{}", add_binary(3, 4));
    println!("{}", set_alarm(false, true));
    println!("{}", dna_strand("TCAG"));
    println!("{}", greet("lasbdjf"));
    println!("{}", bmi(93, 189.0));
}

// https://www.codewars.com/kata/542c0f198e077084c0000c2e/rust
fn divisors(n: u32) -> u32 {
    (1..=n).filter(|x| n % x == 0).count() as u32
}

// https://www.codewars.com/kata/551f37452ff852b7bd000139/rust
fn add_binary(a: u64, b: u64) -> String {
    format!("{:b}", a+b)
}

// https://www.codewars.com/kata/568dcc3c7f12767a62000038/rust
fn set_alarm(employed: bool, vacation: bool) -> bool {
    match (employed, vacation) {
        (true, false) => true, 
        _ => false,
    }
}

// https://www.codewars.com/kata/554e4a2f232cdd87d9000038/rust
fn dna_strand(dna: &str) -> String {
    let mut res:String = String::from("");
    for i in dna.chars() {
        match i {
            'A' => res.push('T'),
            'T' => res.push('A'),
            'C' => res.push('G'),
            'G' => res.push('C'),
            _ => panic!(),
        }
    }
    res.to_string()
}

// https://www.codewars.com/kata/577ff15ad648a14b780000e7/rust
fn greet(language: &str) -> &str {
    return match language {
        "czech" => "Vitejte",
        "danish" => "Velkomst",
        "dutch" => "Welkom",
        "estonian" => "Tere tulemast",
        "finnish" => "Tervetuloa",
        "flemish" => "Welgekomen",
        "french" => "Bienvenue",
        "german" => "Willkommen",
        "irish" => "Failte",
        "italian" => "Benvenuto",
        "latvian" => "Gaidits",
        "lithuanian" => "Laukiamas",
        "polish" => "Witamy",
        "spanish" => "Bienvenido",
        "swedish" => "Valkommen",
        "welsh" => "Croeso",
        "english" => "Welcome",
        _ => "Welcome",
    }
}

// https://www.codewars.com/kata/57a429e253ba3381850000fb/rust
fn bmi(weight: u32, height: f32) -> &'static str {
    let calc = weight as f32 / height.powi(2);
    match calc {
        calc if calc <= 18.5 => "Underweight",
        calc if calc <= 25.0 => "Normal",
        calc if calc <= 30.0 => "Overweight",
        _ => "Obese",
    }
}