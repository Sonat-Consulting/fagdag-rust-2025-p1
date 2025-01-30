// cargo run --example ex_1_ownership

fn take_ownership(list: &Vec<u32>) {
    for item in list {
        println!("{}", item);
    }
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // s1 is no longer valid here
    println!("{}", s2); // This works

    println!("{}", s1); // <---- Compile-time error







    let list = vec![0,1,2,3,4,5];
    take_ownership(list);
    println!("{:?}", list);

}