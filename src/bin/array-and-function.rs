fn main() {
    println!("{}", color_sum([100, 120, 130]));

    show_array(&[1, 2, 3]);
    show_array(&[1, 2, 3, 4, 5]);
    show_array(&[1, 2, 3, 4, 5, 6 , 7, 8, 9, 10]);

    let mut arr: [i32;4 ] = [90 , 43, 58, 230 ] ;
    change_first(&mut arr, 43);

    show_array(&arr);
    
}

// very bad and unpredictable code 
// only for exploring array feature in rust
fn color_sum(color: [u8; 3]) -> i32 {
    let mut sum = 0i32 ;
    for c in color {
        let converted:Result<i32, std::convert::Infallible> = c.try_into() ;
        sum = match converted {
            Ok(n) => sum + n ,
            Err(e) => {
                println!("{}", e);
                sum
            }
        }
    }
    sum
}

// you can write a function and borrow a array 
// with any size and use those values
fn show_array(arr: &[i32]) -> () {
    println!("{:?}", arr);

}

fn change_first(arr: &mut [i32; 4], n : i32) -> () {
    arr[0] = n
} 