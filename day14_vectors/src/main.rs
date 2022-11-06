/* Vectors
    In rust, vectors are defined as a way to store more than one value (of a single data type) in one data structure that puts those values next to each other in memory.
*/

fn main() {
    // To create a vector, it's declared as a variable, with the Vec function, noting its intended data type.
    let mut v1: Vec<i32> = Vec::new();

    // Alternatively, a vector can be declared, without noting its data type, but when it's initialized, it will infer the data type using the values stored
    let v2 = vec![1,2,3,4,5];
    
    // In order to update or insert new values to a vector, the "push" method is used. Note that, as with other languages, new values are put at the end of the list
    v1.push(3);
    v1.push(5);

    // In order to read or access the values of a vector, it can be achieved with either indexing or the "get" method:

        // With indexing, the accessing method is more direct, and if when trying to access an index from out of range, it won't compile
    let third: i32 = v2[2];
    println!("The third element of the second vector is {}", third);

        // With "get", it gains access to a way of accessing indices from out of range, returning "None", and giving access to a "user friendly" way of dealing with errors
    let eighth: Option<&i32> = v2.get(8); //only has 5 indices
    match eighth {
        Some(eighth) => println!("The second element of the second vector is {}", eighth),
        None => println!("Does not exist within range!") //so it will display this error
    }

    // The vector rules also follow the borrowing and ownership rules seen previously, so in this case, when the running program is holding a variable, it cannot be modified
    // until it is used. The following code won't work, so its commented:
    
    /*
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0]; <- we reference the vector here

        v.push(6); <- we try to modify it here

        println!("The first element is: {}", first); <- but until the variable first is used (and so the vector stops being used in memory) it will throw a compiler error
    */

    // Iterating over vectors. Instead of iterating through the indices, Rust allows to iterate directly over the proper elements of the vector.
    for i in &v2 {
        println!("{}", i)
    } // this iterates over all the values of the second vector we created before

    // Rust also allows to iterate and change mutable vectors
    let mut v3: Vec<i32> = vec![30, 40, 55];
    for i in &mut v3 {
        *i += 50; // this will add 50 to each element of the vector
    }

    // Related kata: given 2 arguments, return a vector of the first n multiples of x:
    println!("{:?}", count_by(5, 10));
}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut counter = 1;
    let mut v: Vec<u32> = Vec::new();
    while counter <= n {
        v.push(counter*x);
        counter+=1;
    }
    v
}