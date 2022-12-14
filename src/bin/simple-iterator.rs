struct Counter {
    start: isize,
    stop: isize,
}
impl Iterator for Counter {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.stop {
            self.start += 1 ;
            return Some(self.start);
        }
        None
    }
}
impl Counter {
    fn new(start: isize ,stop: isize) -> Self {
      Self { start, stop }
    }
}
fn main() {
    let counter = Counter::new(100, 10000);
    for c in counter {
        println!("{}", c);
    }
}