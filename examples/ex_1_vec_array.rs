// cargo run --example ex_1_vec_array

fn main() {
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50]; // Fixed-size array

    println!("Array: {:?}", arr);
    println!("Accessing array element at index 2: {}", arr[2]);

    arr[3] = 100; // Modifying an element
    println!("Modified Array: {:?}", arr);

    // Iterating over an array
    print!("Iterating over array: ");
    for num in arr.iter() {
        print!("{} ", num);
    }

    // Array cannot be resized
    //arr.push(60);  // ❌ ERROR: Method doesn't exist

    // Vectors
    let mut vec: Vec<i32> = vec![10, 20, 30, 40, 50]; // Dynamic size

    println!("\nVector: {:?}", vec);
    println!("Accessing vector element at index 2: {}", vec[2]);

    vec[3] = 100; // Modifying an element
    println!("Modified Vector: {:?}", vec);

    // Adding elements dynamically
    vec.push(60);
    vec.push(70);
    println!("Vector after push: {:?}", vec);
}
