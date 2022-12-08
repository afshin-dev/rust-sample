fn add(a: i32, b: i32) -> i32{
    a + b
}
fn main() {
    let sum = add(2, 2);
    // add is a regular function 

    let sub = |a:i32, b: i32| a - b ;
    // sub is a closure 

    // running the both of them is the same 
    println!("sum 2 and 2 : {}", sum);
    println!("sun 10 from 2: {}", sub(10, 2));
}