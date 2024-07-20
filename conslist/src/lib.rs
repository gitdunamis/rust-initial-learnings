use std::rc::Rc;


pub enum ConsList<T> {
    Cons(T, Rc	<ConsList<T>>),
    Nil
}

