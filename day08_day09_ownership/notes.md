# Ownership in Rust

To understand the concept of ownership in Rust, there are 3 rules that must be in place:

* Each value in Rust has an owner;
* There's only one owner at a time;
* Value gets dropped if the owner gets out of scope.

**1. Variable scopes.**

    When using a variable, it is only valid __after__ it's been declared, but __before__ it goes out of scope. Once the end of the scope is reached (ie, after closing brackets), the variable is no longer valid.

        {   // here the variable has not been declared so it's not valid;
            let x = "variable text"; // from this point, the variable x is valid
        }   // x is no longer valid, since the scope has ended

**2. Memory allocation with String Types.**

    When dealing with complex data types, like the "String" type in Rust, the language, since it doesn't possess a garbage collector, has to use another method to deal with memory. In this case, the string, to make it mutable, has to be put into the heap, since at the time of declaring it we don't know how much memory it will occupy, and that heap (as opposed to using the stack to store the variable) allocation must be liberated when the program is done with the string; this is where ownership comes into play: Rust deals with it by clearing all variables when they fall out of scope (their "ownership" ends), by calling the internal function "drop" when it exits.
    One of the ways Rust keeps memory security and stability, its by dropping redundant and/or when they are.. made redundant. Let me explain, when you assign the exact same value to a new variable, and the memory position of that value gets freed by going out of scope, it can lead to code security vulnerabilities when it tries to free the same sector of the memory, twice. Rust solves this by, when that first variable gets re-assigned, the first memory (actually, heap) position its automatically invalidated, so when "both" go out of scope, actually only one is left in the heap, so only one is freed from it. (Detailed better in the rust-lang handbook, chapter 4.1, "double free" error).

    (See example 1 of a error of trying to allocate the same variable twice in memory, in the file "main.rs")

**3. String and memory data duplication with clone method.**

    If there's a situation where a "deep copy" (read: complete, 1-1 copy) of the data contained in the heap (two equal variables), the "clone" method is used. It's a more memory intensive method to use in a program, so usage is discouraged.

**4. Special case: integers (and others).**

    When an integer variable is assigned and stored into memory, then used to reference another variable, since an integer size is known and pushed into memory directly at compile time, it doesn't suffer from the same problems as String variables. The same is true for the following data type cases: (see example 2 of the file "main.rs")
    * boolean data types
    * floating data types
    * character (char) data types
    * tuples data types

    All of these have a known size in memory at compile time, so clone or copy methods are not needed in order to duplicate them in variables.

**5. Ownership in functions.**

    As with variables in the main function (or other parts of the code), when passing a variable to a function in order to use it, it follows the rules previously described, so the ownership ends when the function does, because it goes out of scope.

**6. Returning values.**

    When returning a value from a function, ownership is transferred as if it was being copied into the new scope (in this case the part of the program that got the return of the function).
