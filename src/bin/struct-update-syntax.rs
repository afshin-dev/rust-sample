
#[derive(Debug)]
struct Stc {
    f1: i32,
    f2: bool,
    f3: u8,
    f4: String,
    f5: u128,
}

impl Stc {
    pub fn default() -> Self {
        Self { f1: 10, f2: false, f3: 90, f4: String::from("code"), f5: 9_000_000u128 }
    } 
}

fn main() {
    let stc1 = Stc {
        f4: String::from("script"),
        ..Stc::default()
    };

    println!("{:?}", stc1);
    
    let stc2 = Stc{
        f5 : 1_000_000u128,
        ..stc1
    };
    
    println!("{:?}", stc2);
}