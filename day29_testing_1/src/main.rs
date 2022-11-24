/* Unit testing in Rust
In order to do unit testing in Rust, the language has a few built-in tools, like the configuration conditional compilation attribute,
the "test" attribute, and the assert (which halts if evaluates to false) and assert_eq (which evaluates ) macros.
*/

fn main() {
    let x = 1.4;
    sqrt(x);
}

// To use them in an example, I'm going to write a simple function that takes a number and returns its square root.
// The test will check if the number is positive, and panics if it'is not.

fn sqrt(number: f64)  -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // this imports names from outer scope

    // This test will pass, because we're giving it a correct input.
    #[test] // this indicates to the compiler that we're using the built-in testing function
    fn test_sqrt_1() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    // This test will fail, because we're giving it a negative input.
    #[test]
    fn test_sqrt_2() -> Result<(), String> {
        let x = -1.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}