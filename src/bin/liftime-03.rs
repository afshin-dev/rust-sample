const PROTOCOL: &'static str = "db" ;

fn main() {
    println!("{}", get_protocol());
    let p1 = Person{name: "afshin".to_owned()};
    println!("{}", get_person_name(&p1));
}

struct Person {
    name: String
}

fn get_person_name<'a>(p: &'a Person) -> &'a str {
    &p.name
}

fn get_protocol() -> &'static str {
    PROTOCOL
}