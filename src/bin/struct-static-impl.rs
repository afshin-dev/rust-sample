fn main() {

    // in oop style this is a equivalent of static method
    // fn belongs to struct directly 
    // without createing a instance of it
    Object::static_like_fn();


    // in oop this is a equivalent of instance/object method
    // fn belongs and run on a instance of struct
    let obj: Object = Object { version: 2 };
    obj.method_like_fn();
    obj.show_version();
}

#[derive(Debug)]
struct Object {
    version: isize ,
}

impl Object {
    pub fn static_like_fn() -> () {
        println!("call directly from struct Object", );
    }
    pub fn method_like_fn(&self) -> () {
        println!("call from instance of struct Object");
    }
    pub fn show_version(&self) -> () {
        println!("{}", self.version);
    }
}