fn borrow_clicky(c: &dyn Clicky) {
    c.click();
}
fn move_clicky(c: Box<dyn Clicky>) {
    c.click();
}
// dynamic dispatch allow you to transfer object dynamically
fn main() {
    let m1 = Mouse(MouseKey::L);
    let m2 = Mouse(MouseKey::R);
    let m3= Mouse(MouseKey::M);

    borrow_clicky(&m1);
    borrow_clicky(&m2);
    borrow_clicky(&m3);

    move_clicky(Box::new(m1));
    move_clicky(Box::new(m2));
    move_clicky(Box::new(m3));

    let k1 = Keeboard("w".to_owned());
    borrow_clicky(&k1);
    move_clicky(Box::new(k1));
    // output:
        // #> L of mouse clicked
        // #> R of mouse clicked
        // #> M of mouse clicked
        // #> L of mouse clicked
        // #> R of mouse clicked
        // #> M of mouse clicked
        // #> w clicked
        // #> w clicked
}

trait Clicky {
    fn click(&self) -> () ;
}

#[derive(Debug)]
enum MouseKey {
    R, 
        // right click
    L,
        // left click
    M,
        // middle click
}
struct Keeboard(String) ;

impl Clicky for Keeboard {
    fn click(&self) -> () {
        println!("#> {} clicked", self.0)
    }
}
struct Mouse(MouseKey);

impl Clicky for Mouse {
    fn click(&self) -> () {
        println!("#> {:?} of mouse clicked", self.0);
    }
}