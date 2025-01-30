// cargo run --example ex_4_structs

struct Person {
    name: String,
    age: u8,
    is_active: bool,
}

fn main() {

    // To create an instance of a struct, you specify values for each field.
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        is_active: true,
    };

    // You can access struct fields using dot notation.
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Active: {}", person.is_active);

    // If a struct instance is mutable, you can modify its fields.
    let mut person = Person {
        name: String::from("Bob"),
        age: 25,
        is_active: false,
    };

    person.age = 26; // Modify the age field
}