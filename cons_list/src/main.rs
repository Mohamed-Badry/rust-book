use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn get_current<'a>(list: &'a List) -> Option<(i32, &'a Rc<List>)> {
    match list {
        Cons(current, next) => Some((*current, next)),
        Nil => None,
    }
}

fn main() {
    let list = Cons(4, Rc::new(Cons(2, Rc::new(Nil))));
    println!("list = {:?}", list);
    if let Some((c, n)) = get_current(&list) {
        println!("\nFirst list element {}", c);
        println!("Remaining list {:?}", n);

        if let Some((c2, n2)) = get_current(&n) {
            println!("\nSecond list element {}", c2);
            println!("Remaining list {:#?}", n2);
        }
    }
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
