fn main() {
    let users : Vec<User> = vec![
        User{age: 10, name : "rexa".to_owned()},
        User{age: 23, name : "alex".to_owned()},
        User{age: 32, name : "mima".to_owned()},
    ];
    println!("{:?}", find_by_age(&users, 21));
    println!("{:?}", find_by_name(&users, "alex"));
}

struct User {
    age: i8,
    name: String
}

fn find_by_age(users: &Vec<User>, age: i8) -> Option<bool> {
    for u in users {
        if u.age == age {
            return Some(true)
        }
    }
    None
}
fn find_by_name(users: &Vec<User>, name: &str) -> Option<bool> {
    for u in users {
        if u.name == name {
            return Some(true)
        }
    }
    None
}