// new type pattern allow code consumer like other developer 
// use your struct with convinent manner (easy usage)


mod numbers {
    pub struct NeverZero(i32) ;
    impl NeverZero {
        pub fn new(n: i32) -> Result<Self, String> {
            if n == 0 {
                return Err("n can not be zero".to_owned())
            }
            return Ok(NeverZero(n))
        }
        pub fn get_value(&self) -> i32 {
            self.0
        }
    }
}

fn main() {
    println!("{}", divide(10, numbers::NeverZero::new(2).unwrap()));
    // this code possiblly can crash  
}

fn divide(n1: i32, nz: numbers::NeverZero) -> i32 {
    n1 / nz.get_value()
}