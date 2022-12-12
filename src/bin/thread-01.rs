use std::thread ;
fn main() {
    // let iteration = 10 ;
    let a = thread::spawn(|| {
        for i in 0..=20 {
            println!("A:{}", i);
        }
    });

    let b = thread::spawn(|| {
        for i in 0..=20 {
            println!("      B:{}", i);
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}