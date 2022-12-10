fn main() {
    let usr_id = "jfijrifijepojsi"; // random id
    let usr = User{ id : usr_id};
    println!("{}", usr.id);

    let usr_name = String::from("afshin");
    let person = Person { name : &&usr_name} ;
    println!("{}", person.name);
}

struct User<'a> {
    id: &'a str ,
}
struct Person<'a> {
    name: &'a String
}