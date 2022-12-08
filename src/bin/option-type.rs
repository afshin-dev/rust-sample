fn main() {
    let usr1: User = User {
        age: Some(21),
        name: "john".to_owned(),
    };
    println!("usr: name: {} , age: {:?}", usr1.name, usr1.age);
}

#[derive(Debug)]
struct User {
    age: Option<i8>,
    name: String,
}
