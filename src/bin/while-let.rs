use std::vec;

fn main() {
    let nums = vec![-1, -2, -3 , -4];

    let mut nums_iter = nums.iter();
    
    while let Some(n) = nums_iter.next() {
        println!("n = {}", n);
    }
}