// cargo run --example ex_5_structs

struct Person {
    name: String,
    age: u8,
    is_active: bool,
}
/*
    You can define methods for structs using impl blocks.
    Methods can take &self (immutable reference), &mut self (mutable reference), or self (ownership).
 */


impl Person {
    // Immutable method
    fn display(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }

    // Mutable method
    fn birthday(&mut self) {
        self.age += 1;
    }

    // Associated function (no self parameter)
    fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age,
            is_active: true,
        }
    }
}

fn main() {
    let mut person = Person::new(String::from("Ola"), 40);
    person.display();
    person.birthday();
    person.display();
}