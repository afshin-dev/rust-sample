use std::ops::Add ;


#[derive(Debug)]
struct Speed(i32) ;

impl Add<Speed> for Speed {
    type Output = Speed;
    fn add(self, rhs: Speed) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

fn main() {
    let s1 = Speed(20) ;
    let s2 = Speed(30);
    println!("{:?} + {:?}", s1, s2);
    println!("={:?}", s1 + s2);
}