#![allow(dead_code)]

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("is Engineer"),
        Person::Scientist => println!("is Scientist"),

        Person::Height(i) => println!("has a height of {}", i),
        Person::Weight(i) => println!("has a weight of {}", i),

        Person::Info { name, height } => {
            println!("{} is {} tall", name, height)
        }
    }
}

pub fn test5() {
    let person = Person::Height(18);
    let amira = Person::Height(10);

    let dave = Person::Info {
        name: "Dave".to_owned(),
        height: 72,
    };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

pub fn test5_1() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money"),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn test5_2() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
