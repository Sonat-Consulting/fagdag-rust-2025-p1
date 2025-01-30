// cargo run --example ex_7_optionals

/**
    Rust uses the Option<T> enum to represent optional values, meaning a value that might or might not exist.
    Instead of using null, Rust explicitly defines the possibility of absence using Option<T>.

    enum Option<T> {
        Some(T), // Contains a value
        None,    // Represents the absence of a value
    }

    Prevents null pointer errors.
    Forces handling of missing values at compile time.
    Improves code safety and readability.
**/

fn square_root(value: f64) -> Option<f64> {
    if value < 0.0 {
        None // No square root for negative numbers
    } else {
        Some(value.sqrt()) // Compute square root
    }
}

fn main() {
    let num = 16.0;

    match square_root(num) {
        Some(result) => println!("Square root: {}", result),
        None => println!("No real square root exists"),
    }

    let invalid_num = -4.0;

    match square_root(invalid_num) {
        Some(result) => println!("Square root: {}", result),
        None => println!("No real square root exists"),
    }

    // If you don't want to handle the potential None case. This will panic if `None`
    let sqrt = square_root(5.0).unwrap();

    // Provide default if None
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x: {}", x.unwrap_or(0)); // Returns 5
    println!("y: {}", y.unwrap_or(0)); // Returns 0 (default)
}