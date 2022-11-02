# Referencing variables and borrowing

## Referencing

In order to pass references into functions, in Rust, the ampersand (&) is used. This way, the function can use a variable without taking actual ownership of it, this way it won't cause any problems when it goes out of scope, since it's not the actual variable, but a _reference_ to it. (See example 1 on main.rs)

This also means that, in Rust, by default, you cannot modify or work with the borrowed (referenced) variable, unless you specify that you can do it, using a _mutable reference_.

## Mutable references

To pass a mutable reference, both the original variable and the reference to it, have to include the "mut" keyword. This way changes can me made upon the original variable, again, without actually taking ownership of it. (See example 2 on main.rs)

Mutable references also have a tight restriction: only one reference of a value can be active at a time; if you have a mutable reference to a value, you cannot reference that value again.

Similarly, in Rust, there cannot be mutable and immutable references to the same value. Although, when the original (immutable) reference data falls out of scope, then it can be used in a mutable reference, it's not _overlapping_.

## Dangling references

Unlike other languages that manage pointers and references, Rust doesn't allow to have a dangling pointer. At compile time, it guarantees that the reference will not go out of scope before the data the referenced data does.

In general, the best practice would be to return the data directly, and not a reference to it, and Rust only allows that to happen, hence preventing dangling pointer problems.

## General rules for referencing

1. There can only be, at any point, either _one MUTABLE_ reference, or _any number_ of _IMMUTABLE_ refrences.
2. References must always be valid ones.
