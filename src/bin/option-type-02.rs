struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}
fn main() {
    let response: Survey = Survey {
        q1: Some(11),
        q2: Some(true),
        q3: Some("yes".to_owned()),
    };
    match response.q1 {
        Some(n) => println!("{:?} is for q1", n),
        None => println!("not answered"),
    }

    match response.q2 {
        Some(stated) => println!("{:?} is for q2", stated),
        None => println!("not answered"),
    }

    match response.q3 {
        Some(answer) => {
            if answer == String::from("yes") {
                println!("is {} for q3", answer);
            }
            if answer == String::from("no") {
                println!("is {} for q3", answer);
            }
        }

        None => println!("not answered"),
    }
}
