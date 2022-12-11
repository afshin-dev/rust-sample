// new type pattern allow code consumer like other developer 
// use your struct with convinent manner (easy usage)

struct NeverZero(i32) ;
impl NeverZero {
    fn new(n: i32) -> Result<Self, String> {
        if n == 0 {
            return Err("n can not be zero".to_owned())
        }
        return Ok(NeverZero(n))
    }
}

fn main() {
    println!("{}", divide(10, NeverZero::new(0).unwrap()));
    // this code possiblly can crash  
}

fn divide(n1: i32, nz: NeverZero) -> i32 {
    n1 / nz.0
}