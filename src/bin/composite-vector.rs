use std::vec;

fn main() {
    let mut jobs: Vec<Box<dyn Runnable>> = vec![] ;
    let vm = Box::new(VMMachine{});
    let c1 = Box::new(Container{});
    let c2 = Box::new(Container{});

    jobs.push(vm);
    jobs.push(c1);
    jobs.push(c2);

    for job in jobs {
        job.run();
    }
        // out put 
    //     #> vm started
    // #> container started
    // #> container started
}

trait Runnable {
    fn run(&self) ;
}

struct VMMachine{

}
impl Runnable for VMMachine {
    fn run(&self) {
        println!("#> vm started");
    }
}
struct Container {

} 

impl Runnable for Container {
    fn run(&self) {
        println!("#> container started");
    }
}