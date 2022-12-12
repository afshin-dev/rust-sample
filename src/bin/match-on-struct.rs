struct Person {
    name: String,
    age : u8,
}

fn main() {
    let person = Person { age: 31 , name: "rexa".to_owned()};

    match person {
        Person{age: 10..=20 , ..} => println!("10 to 20"),
        Person {  age: 30..=33 | 38..=40, .. } => println!("30 to 33 or 38 to 40"),
        Person { .. } => println!("person: {}", person.name),
    }
}