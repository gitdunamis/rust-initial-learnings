use std::rc::Rc;

use conslist::ConsList::{Cons, Nil, self};

fn main() {
    println!("Hello, world!");
    let tree_one = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let three = Cons(3, Rc::clone(&tree_one));
    let four = Cons(4, Rc::clone(&tree_one));

    println!("Number of borrows to original data: {}", Rc::strong_count(&tree_one))
}
