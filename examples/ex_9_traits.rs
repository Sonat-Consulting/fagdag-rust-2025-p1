// cargo run --example ex_9_traits

/*
    Traits define shared behavior . They allow you to specify a set of methods that a type must implement,
    similar to interfaces in other languages (like Java or Go).
 */

trait Greet {
    fn greet(&self); // Method signature (without implementation)

    // w/ default implementation
}

// Implement the trait for a struct
struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

fn main() {
    let p = Person { name: String::from("Alice") };
    p.greet(); // Calls the trait method
    //p.default_greet();
}