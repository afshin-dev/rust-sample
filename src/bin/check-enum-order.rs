


#[derive(Debug, PartialEq, PartialOrd)]
enum Floor {
    One,
    Two,
    Three,
    Four
}

fn main() {
    // this is a bad code 
    // because using 4 Enum for compare ordering of two enum but this is a syntax and feature test
    println!("{:?} is higher than {:?} = {}", Floor::Two, Floor::One, is_higher(Floor::Two, Floor::One));
    println!("{:?} is higher than {:?} = {}", Floor::Four, Floor::Three, is_higher(Floor::Four, Floor::Three));
    println!("{:?} is higher than {:?} = {}", Floor::Two, Floor::Four, is_higher(Floor::Two, Floor::Four));
}

fn is_higher(f1: Floor, f2  : Floor ) -> bool {
    if f1 > f2 {
        return true
    }
    false 
}