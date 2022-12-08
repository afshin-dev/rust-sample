fn main() {
    // numeric range 
    let range = 1..10 ;
    for item in range {
        println!("{:?}" , item);
    }

    // alphabetic like range 
    let range = 'a'..='i' ;
    for item in range {
        println!("{:?}", item);
    }
}