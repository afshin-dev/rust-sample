fn main() {
    let numbers = vec![1, 10, 100, 1000, 10_000, 100_000];
    let plus_one: Vec<i32>  =numbers
        .iter()
        .map(|number| number + 1)
        .collect() ;

    dbg!(plus_one);
    // result
//     [src\bin\map-vector-item.rs:8] plus_one = [
//     2,
//     11,
//     101,
//     1001,
//     10001,
//     100001,
// ]
}