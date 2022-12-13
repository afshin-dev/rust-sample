use std::cell::Cell ;


struct Agreement {
    singed: Cell<bool>
}

impl Agreement {
    fn sign(&self) {
        self.singed.set(true);
    }
    fn unsign(&self) {
        self.singed.set(false);
    }
    fn is_signed(&self) -> bool{
        self.singed.get()
    }
}
fn main() {
    let agreement_with_you = Agreement{ singed : Cell::new(false)} ;
    println!("agreement with you is signed : {}", agreement_with_you.is_signed());

    agreement_with_you.sign();
    println!("agreement with you is signed : {}", agreement_with_you.is_signed());
    
    agreement_with_you.unsign();
    println!("agreement with you is signed : {}", agreement_with_you.is_signed());


}