fn main() {
    println!("{}", math(1, 1, Box::new(|x: i32, y: i32| -> i32 { x + y})));
    println!("{}", math(1, 1, Box::new(|x: i32, y: i32| -> i32 { x - y})));
    println!("{}", math(1, 1, Box::new(|x: i32, y: i32| -> i32 { x * y})));
    println!("{}", math(200, 2, Box::new(|x: i32, y: i32| -> i32 { x / y})));
}

fn math(x: i32 , y: i32 , op:Box<dyn Fn(i32,i32) -> i32 >) -> i32 {
    op(x, y)
}