fn main() {
    let speed:u8 = 40 ; //1 to 100
    let lvl = Level::Begginer ;
    match speed {
        s if (s <=  40 && lvl == Level::Begginer) => println!("it begginer player"),
        s if (s > 80  && lvl == Level::Intermediate) => println!("good player"),
        s if(s >= 60 && s <= 80 && lvl == Level::Advanced) => println!("professional"),
        s => println!("not determined yet: {}", s),
    }
}

#[derive(PartialEq)]
enum Level {
    Begginer,
    Intermediate,
    Advanced,
}