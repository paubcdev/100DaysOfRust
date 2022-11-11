fn main() {
    println!("{}", camel_case("TextoDePrueba"));
    println!("{}", valid_parentheses("()()()"))
}

// https://www.codewars.com/kata/5208f99aee097e6552000148
fn camel_case(s: &str) -> String {
    let mut resu: String = String::new();
    for i in s.chars() {
        if i.is_uppercase() {
            resu.push(' ');
        }
        resu.push(i);
    }
    resu
}

// https://www.codewars.com/kata/52774a314c2333f0a7000688
fn valid_parentheses(s: &str) -> bool{
    let mut list = 0;
    for i in s.chars() {
        if i == '(' {
            list += 1;
        } else if i == ')' {
            list -= 1;
            if list < 0 {
                return false;
            }
        }
    }
    list == 0
}