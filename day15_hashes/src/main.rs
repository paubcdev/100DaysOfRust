/* Hash maps
A hash map, similar to a dictionary in Python or an object in Javascript, is a data collection, that stores a key "K" tied to a value "V", which maps and binds them together.
This way it's easier to search for a value by using the key associated with it, instead of the index in a vector. All keys and values must be of the same data type as all others.
*/

fn main() {
    use std::collections::HashMap; // The standard library must be called and referenced in order to use hash maps, since its the least used collection generally in the language.

    // To create a hash map, use the function new() and then add elements using .insert(). In this case, the data types are infered by the compiler, but they can be explicitly set.
    let mut cars = HashMap::new(); // [...] cars: HashMap<String, i32> = [...]

    // In this example, we create a hash map that holds the color of some cars and the number of each color
    cars.insert(String::from("Blue"), 14);
    cars.insert(String::from("Red"), 18);
    cars.insert(String::from("Green"), 21);

    // One way of accessing values within a hash map is to provide the key to a get method
    let color = String::from("Blue");
    let number = cars.get(&color).copied().unwrap(); // note that the .copied() method is to return an i32 value instead of an &i32
    println!("The number of blue cars is: {}", number);

    // Another way of accesing the hash map is to iterate through it like we'd do in a vector
    for (key, value) in &cars {
        println!("{}: {}", key, value); // note that this way will print it in a random order
    }

    // To overwrite a value in a hashmap, simply call the insert method on an existing key
    cars.insert(String::from("Blue"), 30); // previously it was 14, now it will be 30
    println!("{:?}", cars);

}

/*fn points(games: &[String]) -> u32 {
    for i in games {
        todo!()
    }
}*/