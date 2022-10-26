// Variables 2: data types, constants, shadowing, tuples

fn main() {
/*
Data types in Rust can be integers (signed or unsigned), float points and booleans;
integers range from 8 bit to 128 bit, and can be signed (i32), or unsigned (u64);
float point types can be single precision (f32) or double precision (f64);
booleans can either be "true" or "false".
*/
    let a: i8 = -54; // can be positive or negative
    let b: u16 = 4000; // must only be positive
    let c: f32 = 2.0;
    let d = true;

/*
Constants and immutable variables are similar but not exactly the same;
constants cannot be assignged the "mut" modifier, and ALWAYS must be assigned a value type;
constants can be declared in any scope, even globally;
"const" is used to declare constants.
*/
    const GRAVITY_IN_MS: f32 = 9.8;

    println!("The speed of gravity in meters per second is: {}", GRAVITY_IN_MS);

/*
"Shadowing", or using already-declared variables;
when a variable already declared and used is declared again, the compiler uses it again,
in the scope that is declared;
this way, shadowed variables must only be changed when declared with "let", and its type
can be changed, unlike with 'mut' mutable variables using "mut", which don't allow type
change.
*/
    let spaces = "    ";
    let spaces = spaces.len(); // if we tried to do this with a "mut" variable, compiler
                               // would throw an error, since once declared that way,
                               // its type cannot change.

/*
Tuples in rust are a way of organizing values of different type into one;
tuples have a fixed length, once declared cannot be made bigger or smaller.
*/
    let tup1 = ('a', 32);

/*
Tuples are "binded" together, so to extract and use its individual elements, different
variables must be declared independently.
*/
    let (x, y) = tup1;
    println!("The value of the first tuple element is: {x}");
}
