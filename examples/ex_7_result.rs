use std::error::Error;

// cargo run --example ex_7_result

/**

    enum Result<T, E> {
        Ok(T),  // Contains the successful result
        Err(E), // Contains the error value
    }



    Use Option<T> when absence is a valid state (e.g., searching for an item in a list).
    Use Result<T, E> when an operation can fail and needs an error message.

**/

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero")) // Error case
    } else {
        Ok(a / b) // Success case
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = divide(10, 2);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // Panics if Err
    let result = divide(10, 2).unwrap();

    // Provide default value
    let result = divide(10, 2).unwrap_or(0);

    let result = divide(10, 0)?;

    Ok(())

}