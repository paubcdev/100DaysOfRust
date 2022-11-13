/* A struct (for "structure") is a data type, set by the programmer, that groups together certain related values to form a "super-type" or gropus them in a meaningful way.
Drawing parallelisms with OOP languages, it's similar to an object's data attributes. 

At first glance, structs seem very similar to tuples, in that they contain multiple related values, and those values can be of different types. The first difference is that
in a struct each value is named and doesn't matter the order they are contained in, since they are referenced by name, not position; this way they are fundamentally, more
flexible. */

// to define a structure, use the "struct" keyword, followed by the name, then brackets; inside place the data names and their types. Each one is called a "field"
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // its recommended that the name of the struct is relevant to the data values placed inside it, and that the data names are clear and concise

fn main() {

// After defining the structure, we create an instance of it by giving it a name, calling the struct, and srrring each field with a value -using key: value pairs-
    let mut user1 = User {
        email: String::from("test@example.com"),
        active: true,
        sign_in_count: 1,
        username: String::from("testuser"),
        // the order of the key-value pairs doesn't need to match the order of the structure at time of definition
    };
    // NOTE that the whole instance must (or must not) be mutable; Rust doesn't allow just certain fields to be mutable

    // to access or modify the data inside a struct, "dot" notation is used
    println!("{}", user1.email);
    user1.username = String::from("anotherusername");
    println!("{}", user1.username);

    // when most (or some) fields can be reused from other instances, there's a way to specify so in new instances of the struct
    let user2 = User {
        email: String::from("test2@example.com"),
        username: user1.username,
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    // BUT there's even a simpler way of achieving so, by using a method called "struct update syntax"
    let user3 = User {
        email: String::from("example@example.com"),
        // now by calling the syntax "..", it tells the program to reuse the same data in the specified instance unles otherwise defined
        ..user2
    };
    println!("User 2 email: {}", user2.email);
    println!("User 3 email: {}", user3.email)
    // even though the rest of the fields are copied from user two, since we specified the email, it is reflected
}
