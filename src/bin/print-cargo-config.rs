const CONFIG: &'static str = include_str!("../../cargo.toml");

fn main() {
    println!("{}", CONFIG);
}