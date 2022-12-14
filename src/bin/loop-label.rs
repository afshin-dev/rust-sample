fn main() {
    'outer: loop {
        println!("outer loop run.");
        'inner: loop {
            println!("\t\tinner loop run");
            break 'outer;
        }
    }
    println!("breaked from outer lable");
}