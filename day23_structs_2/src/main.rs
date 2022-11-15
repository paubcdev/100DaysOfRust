/* Structs 2: Methods

In order to talk about methods, it's useful to think of it as functions that perform instructions on a struct's instance (that's why it's often bundled together). 
To use methods, they are declared after a struct and reference it with the keyword "impl", then the function that changes the struct. Step by step:
1. Declare the struct;
2. use the impl keyword with the same name as the one given to the struct;
3. write a function inside the implied (impl) body but;
4. write "self", "&self" or "&mut self" (as needed by borrowing rules) as the first parameter of the function;
5. call and use the method.
Example as follows.
*/

struct Student {
    id: i32,
    name: String,
    marks: i32,
}

impl Student {
    fn penalize_by_5(&mut self){
        self.marks -=5
    }
}
// This method is used in the Student struct to automatically substract 5 points from the "marks" field.

fn main() {
    let mut s1 = Student {
        id: 1,
        name: String::from("Pau"),
        marks: 95,
    };
    println!("Student id and name: {}, {}", s1.id, s1.name);
    println!("Original marks: {}", s1.marks);

    s1.penalize_by_5();

    println!("New marks: {}", s1.marks)
}


/* All functionms defined inside an impl block are called "associated functions", that's because they're associated with the struct named after the impl.
Functions that don't have the "self" parameter are considered regular and not methods, because they don't need an instance of that struct type to work with.

Associated functionw that are not methods, are commonly used as constructors that will end up returning a new instance of the struct.*/