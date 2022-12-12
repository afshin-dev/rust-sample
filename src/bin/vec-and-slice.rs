fn main() {
    let chars = vec!['a', 'b', 'c', 'd'];
    let ab: &[char] = &chars[0..=1] ;
    let cd: &[char] = &chars[2..=3];

    for ch in ab {
        println!("{}", ch);
    }

    println!("{}", "-".repeat(50));

    for ch in cd {
        println!("{}", ch);
    }
}