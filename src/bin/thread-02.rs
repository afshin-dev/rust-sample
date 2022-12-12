use std::thread ;
use std::time::Duration;

fn main() {
    let t1 = thread::spawn(|| -> i32 {
        thread::sleep(Duration::from_secs(5));
        11
    });

    match t1.join() {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{:?}", e)
    }
}

