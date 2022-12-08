fn main() {
    let numbers : Vec<i32> = vec![1, 90, 250] ; // random number 
    let res: Option<&i32> = numbers
        .iter()
        .find(|n| n == &&90);

    if res.is_some() {
        println!("{:?} is find" , res);
    }    
}

