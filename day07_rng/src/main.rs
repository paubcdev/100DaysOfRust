// Crates/use of cargo as a "library/module" management tool.

/* To use a module or a library that is contained in the crate repositories,
you have to edit the "Cargo.toml" file, and add the name and version of the module
you want to use, then run "cargo build" to check and update its own dependencies */

use rand::Rng; // in this case its using the random package

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    println!("Fixed randomly generated number: {}", n1);

    let n2: u16 = rng.gen_range(0..100);
    println!("Randomly generated number from a range: {}", n2);
}
