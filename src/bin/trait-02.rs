fn main() {
    (Simple{}).incl();
    
    // let mut n = -1 ;
    // iinc(&mut n);
    // println!("{}", n);
}

// fn iinc(n: &mut i32) { // inplace increment
//     println!("{}", n);
//     println!("{}", *n);
//     *n = *n+1 ;
// }
trait Incl {
    fn incl(&self){
        println!("incl running...");
    }
}

struct Simple {}
impl Incl for Simple {
    
}

