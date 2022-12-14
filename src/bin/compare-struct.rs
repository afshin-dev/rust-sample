#[derive(Debug, PartialEq)]
struct User {
    id: i128,
    name: String,
}

fn main() {
    // considered bad code 
    println!(
        "{:?} is a {:?} : {}",
        User {
            id: 1,
            name: "a".to_owned()
        },
        User {
            id: 1,
            name: "a".to_owned()
        },
        is_same(
            User {
                id: 1,
                name: "a".to_owned()
            },
            User {
                id: 1,
                name: "a".to_owned()
            }
        )
    );
}

fn is_same(u1: User, u2: User) -> bool {
    if u1 == u2 {
        return true;
    }
    false
}
