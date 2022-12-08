use std::collections::VecDeque ;

fn main() {
    let mut vd:VecDeque<i32> = VecDeque::new();
    vd.push_front(10);
    vd.push_front(100);
    dbg!(vd);
}