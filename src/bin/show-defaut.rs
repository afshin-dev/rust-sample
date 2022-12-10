use std::fmt::Debug;

fn main() {

}

fn show_default<T>(o: T) -> i32
    where
    T: Default + Debug
{
    println!("{:?}", o);
    1
}