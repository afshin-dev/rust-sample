enum Status {
    Info,
    Warn,
    Error(i32),
}

fn main() {
    use Status::{Error, Warn, Info};
    let status = Status::Error(3) ;
    match status {
        Error(e @ 3) => {
            println!("error {}", e);
        },
        Error(e @ 10..=14) => {
            println!("error 10..=14 : {}",e );
        },
        Error(e @ 1 | e @ 2) =>  {
            println!("1 or 2: actual {}", e);
        },
        Error(e) => {
            println!("other: {}", e);
        },
        Warn => {
            println!("warning");
        },
        Info => {
            println!("Info");
        }
    }
}