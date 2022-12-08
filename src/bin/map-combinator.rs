fn get_one_or_none(one: bool) -> Option<i8> {
    if one == true {
        return Some(1i8)
    }
    None
}

fn main() {
    // play with one argument of function for different result
    let two: Option<i8> = get_one_or_none(true)
        .map(|one| one + 1  );
    match two {
        Some(n) => {
            println!("n = {}", n);
        }, 
        None => {
            println!("None");
        }
    }    
}