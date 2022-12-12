enum Element {
    Fire, 
    Water,
    Soil,
    Wind,
}

fn main() {
    use Element::{Water, Wind, Fire, Soil} ;

    let element = Element::Wind ;
    match element {
        Fire | Water => println!("dangerous Element"),
        Soil => println!("good element but be cautious"),
        Wind => println!("use it to fly with it"),    
    }
}