fn main() {
    let  stream = data().chunks(2);
    for chunk in stream {
        // println!("{:?}", chunk);
        let sum = sum_chunk(chunk);
        println!("{}", sum);
    }
    // sum_chunk(&[1, 1 , 1]);
        // undo this line to get unreachable error 
}

fn data() -> &'static [u64] {
    &[20, 21, 43, 45, 90, 32, 44, 1,11,111,44,1]
    
}

fn sum_chunk(chunk: &[u64]) -> u64 {
    match chunk {
        [a, b] => a + b ,
        [a] => *a,
        [] => 0 ,
        [..] => unreachable!("allowed size of chunk: 0 , 1 , 2") ,
    }
}