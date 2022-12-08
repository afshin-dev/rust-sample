fn main () {
    fmt::print("afshin.");
}

mod fmt {
    pub fn print(s: &str) -> () {
        println!("{}", s);
    }
}