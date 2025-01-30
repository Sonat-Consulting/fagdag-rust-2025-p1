
/**
Macros in Rust allow you to write metaprogramming—code that writes other code.
They are more powerful than functions because they can take in a variable number of arguments, work at compile time, and generate code dynamically.

**/

//#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Expands into:

/*
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
} */

fn main() {
    let p = Person { name: "Alice".into(), age: 30 };
    println!("{:?}", p);
}