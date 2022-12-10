fn main() {
    (PlayGround{game: Football{duration: 90}}).game_info();
    (PlayGround{game: Tenis {round : 12}}).game_info(); // arbitary round choosen
    // Output:
        // football
        // tenis
}

trait Game {
    fn name(&self) -> String ;
}

struct PlayGround<T> 
    where
        T: Game
{
    pub game: T ,
}

// target section of syntax and feature that tested in this file
impl<T> PlayGround<T> 
    where
        T: Game
{
    fn game_info(&self) {
        println!("{}", self.game.name());
    }
}

struct Football {
    duration: i64 , // in min
        // no problem never used field

}

impl Game for Football {
    fn name(&self) -> String {
         "football".to_owned() 
    }
}
struct Tenis {
    round: i32 , 
        // no problem never used field
}

impl Game for Tenis {
    fn name(&self) -> String {
        "tenis".to_owned()
    }
}