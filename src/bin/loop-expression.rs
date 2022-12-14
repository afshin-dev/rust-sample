fn main() {
    let mut x = 0 ;
    let last  = 'o: loop {
        x  += 1 ; 

        'i: loop {
            x *= 2 ;
            if x > 200 {
                break 'i; 
            }
        }

        x *= 2 ;

        if x > 10000 {
            break 'o x;
        }


    };
    println!("{}",last);
}