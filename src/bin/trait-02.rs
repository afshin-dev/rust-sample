fn main() {
    (Simple{}).incl();
}

trait Incl {
    fn incl(&self){
        println!("incl running...");
    }
}

struct Simple {}
impl Incl for Simple {
    
}

