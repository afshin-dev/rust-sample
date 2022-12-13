use crossbeam_channel::{unbounded} ;
use std::thread ;

fn main() {
    let (a, b) = unbounded(); 
    let t1 = thread::spawn(move || {
        loop {

            match b.recv() {
                Ok(m) =>{ 
                    println!("{}", m);
                    break;
                },
                Err(e) => println!("{}", e),
            }
        }
    });

    a.send("from main to thread".to_owned()).unwrap();

    t1.join().unwrap();
}