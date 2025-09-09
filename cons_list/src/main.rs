use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn get_current<'a>(list: &'a List) -> Option<(i32, &'a Box<List>)> {
    match list {
        Cons(current, next) => Some((*current, next)),
        Nil => None,
    }
}

fn main() {
    let list = Cons(4, Box::new(Cons(2, Box::new(Nil))));
    println!("b = {:?}", list);
    if let Some((c, n)) = get_current(&list) {
        println!("\nFirst list element {}", c);
        println!("Remaining list {:?}", n);

        if let Some((c2, n2)) = get_current(&n) {
            println!("\nSecond list element {}", c2);
            println!("Remaining list {:#?}", n2);
        }
    }
}