use std::{io} ;

fn main() {
    let mut buffer = String::new();
    let line = io::stdin().read_line(&mut buffer);
    match line {
        Result::Ok(read_size) => {
            println!("{} bytes \n input = {}",read_size, buffer);
        },
        Result::Err(err) => {
            println!("{:?} ", err);
        }
    }


    let mut s = String::new() ;
    run(&mut s);
    println!("{}", s);
}

fn run(s: &mut String) {
    s.insert_str(0, "changed a mutable string");
}