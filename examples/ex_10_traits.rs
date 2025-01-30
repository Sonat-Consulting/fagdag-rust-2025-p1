// cargo run --example ex_10_traits

trait Shout {
    fn shout(&self) -> String;
}

// Implementing the trait for the built-in type `String`
impl Shout for String {
    fn shout(&self) -> String {
        self.to_uppercase() + "!!!"
    }
}

fn main() {
    let text = String::from("hello");

    // print ...

}