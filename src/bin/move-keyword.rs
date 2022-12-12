fn main() {
    let name: String = String::from("afshin");
    let function = move || { // move external value to closure
        println!("{}", name); 
    } ; 
    // name no longer available here 
    run_it(Box::new(function));
}

fn run_it(func: Box<dyn Fn() -> () >) -> () {
    func();
}