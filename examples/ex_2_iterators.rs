// cargo run --example ex_2_iterators

fn main() {
     let v = vec![1, 2, 3];

     // Immutable iterator
     for item in v.iter() {
         println!("{}", item);
     }

     // Mutable iterator
     let mut v = vec![1, 2, 3];
     for item in v.iter_mut() {
         *item += 1;
     }

     // Consuming iterator
     for item in v.into_iter() {
         println!("{}", item);
     }

     // Error
     for item in v {
         println!("{}", item);
     }

    // Collect
     let v = vec![1, 2, 3, 4, 5];

     let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
     println!("{:?}", doubled); // [2, 4, 6, 8, 10]


     // Infinite loop
     let mut i = 0;
     loop {
         i += 1;
         if i == 5 {
             break;
         }
         println!("Hello");
     }

     // Named loops
     'outer: for i in 0..10 {
         'inner: for j in 0..i {
             println!("{}", j);
             if j >= 3 {
                break 'outer;
             }
         }
     }
 }