fn main()  {
    let items: [i32; 4] = [12, 14, 10, 19];
    println!("{:?}", items);

    for i in items {
        println!("{}", i);
    }
}