use std::{ops::Deref, fmt::Debug};

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    let y = ABox::new(x);

    println!("x -> {x}");
    println!("y -> {y:?}");
    println!("*y -> {}", *y);

    assert_eq!(5, *y);

    println!("Did we drop the first one first");

    drop(y);

    print_name(&ABox::new("James"));


    println!("Did we drop all here");

}


#[derive(Debug)]
struct ABox<T> (T);

impl<T> ABox<T> {
    fn new(value: T) -> ABox<T> {
        ABox(value)
    }
}

impl<T> Deref for ABox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for ABox<T> {
    fn drop(&mut self) {
        println!("Dropping this ...bye!!!")
    }
}

//impl<T> Drop for ABox<T> {
//    fn drop(&mut self) {
//        println!("Dropping this:...bye!!!")
//    }
//}

//impl<T> Display for ABox<T> where T:Display {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.write_fmt(format_args!(""))
//    }
//}

fn print_name(name: &str) -> () {
    println!("This is ur name: {name}")
}