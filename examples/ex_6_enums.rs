// cargo run --example ex_6_enums

enum Fruit {
    Apple,
    Banana,
    Orange
}

enum Vehicle {
    Car {
       hp: u32,
    },
    Motorcycle {
        hp: u32
    },
    Sailboat
}


fn main() {
    let f = Fruit::Banana;

    match f {
        Fruit::Apple => println!("It's an apple"),
        Fruit::Banana => println!("It's a banana"),
    }
    
 
    let v = Vehicle::Car { hp: 150};

    match v {
        Vehicle::Car {hp} => println!("The car has {} hp!", hp),
        Vehicle::Motorcycle {hp} => println!("The MC has {} hp!", hp),
        Vehicle::Sailboat => println!("The sailboat has no hp!")
    }
}