// cargo run --example ex_5_enums
enum Fruit {
    Apple,
    Banana,
    Orange
}
fn main() {
    let f = Fruit::Banana;

    match f {
        Fruit::Apple => println!("It's an apple"),
        Fruit::Banana => println!("It's a banana"),
    }

}