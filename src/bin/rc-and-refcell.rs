use std::rc::Rc ;
use std::cell::RefCell ;

struct MainWatcher {
    lst: Rc<RefCell<Vec<String>>>
}
struct SharedWatcher {
    lst : Rc<RefCell<Vec<String>>>,
}
fn main () {
    let list: Vec<String> = vec![
        String::from("mail to boss"),
        String::from("secret letter to my sister"),
    ];
    let list = RefCell::new(list);

    let list = Rc::new(list);

    println!("{:?}", list);

    let mw = MainWatcher{ lst : Rc::clone(&list)};
    let sw = SharedWatcher{ lst : Rc::clone(&list)};
    {
        let mut l = mw.lst.borrow_mut();
        l.push("main watcher add something".to_owned());
    }
    // {
    //     println!("{:?}", sw.lst);
    // }
    {
        let mut l = sw.lst.borrow_mut();
        l.push("shared watcher add something".to_owned());
    }

    {
        println!("sw: {:?}", sw.lst);
        println!("mw: {:?}", mw.lst);

    }

}