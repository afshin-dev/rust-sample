fn main() {
    let prime_user_list = PrimUserList{lst : get_users()};
    let mut under_30_user  = TempUserList{lst: Vec::new()};

    under_30_user.lst = prime_user_list.lst.iter().filter( |u| {
        match u.age {
            Some(a) => {
                if a < 30 {
                    true
                }else {
                    false
                }
            },
            None => {
                false
            }
        }
    }).collect();


    for u in under_30_user.lst {
        println!("{} {} {:?}", u.name, u.family, u.age)
    }
    // output 
        //     bb b Some(12)
        // dd d Some(24)
        // ee e Some(26)
}


fn get_users() -> Vec<User> {
    vec![
        User{age: Some(50), family: "a".to_owned(), name: "aa".to_owned()},
        User{age: Some(12), family: "b".to_owned(), name: "bb".to_owned()},
        User{age: Some(31), family: "c".to_owned(), name: "cc".to_owned()},
        User{age: Some(24), family: "d".to_owned(), name: "dd".to_owned()},
        User{age: Some(26), family: "e".to_owned(), name: "ee".to_owned()},
    ]
}
struct User {
    age: Option<i8>,
    family: String,
    name: String,
}

struct PrimUserList {
    lst: Vec<User>
}

// this struct must create from PrimUserList
// for any temporary calculation or filtering of user 
struct TempUserList<'a> {
    lst: Vec<&'a User>
}