fn main() {
    priority_tab(RegularGuest{});
    priority_tab(SpecialGuest{});
}

fn priority_tab<T: Priority>(guest: T) {
    println!("{:?}", guest.get_priority());
}

#[derive(Debug)]
enum ServicePriority {
    Standard,
    High,
}
trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct SpecialGuest ;

impl Priority for SpecialGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High 
    }
}

#[derive(Debug)]
struct RegularGuest;

impl Priority for RegularGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}