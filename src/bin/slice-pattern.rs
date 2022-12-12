fn main() {
    let nums = vec![3, 5, 11, 1234, 256, 325, 145];

    match nums.as_slice() {

        [first@ 3, second @ 5, third @ 10, ..] => {
            println!("{} {} {}", first, second, third);
        },

        [first @ 8, second @ 100, ..] => {
            println!("{} {} ", first, second);
        },
      
        [] => {
            println!("empty");
        },

        [..] => {
            println!("other");
        },
    }
}