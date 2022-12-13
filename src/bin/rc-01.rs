use std::rc::Rc ;


struct Parent {
    name: String ,
    id : i128,
}

struct Child {
    name: String,
    id: i128,
    parent: Rc<Parent>,
}
fn main() {
    // create a reference count of parent
    let p1 = Rc::new(Parent { name : "p1".to_owned(), id : 1_i128});

    // assign it to child 
    let child_1 = Child { name  : "c1".to_owned() , id : 2_i128, parent : Rc::clone(&p1)  };
    let child_2 = Child { name  : "c2".to_owned() , id : 3_i128, parent : Rc::clone(&p1)  };

    // eliminate of parent 
    drop(p1);

    println!("child = {}  parent {}",child_1.name ,child_1.parent.name);
    println!("child = {}  parent {}",child_2.name ,child_2.parent.name);
}   