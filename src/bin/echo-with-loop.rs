use std::io::stdin ;
fn main() {
    loop {
        let mut buffer = String::new() ;
        let line = stdin().read_line(&mut buffer);
        match line  {
            Ok(size) => {
                println!("{} byte reads = {}", size, buffer);
                if &buffer.trim().to_lowercase() == "exit" {
                    break;
                }

            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}