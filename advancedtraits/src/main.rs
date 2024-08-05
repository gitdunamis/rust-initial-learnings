
fn main() {
    println!("Hello, world!");
    
    let string = StringList::new(&["first", "second", "third"]);
    
    println!("List content is: {string:?}");
    
    while let Some(s) = string.next() { //this loops forever
        println!("s: {s}")
    }
}

trait Iterator {
    type Item;
    
    fn next(&self) -> Option<&Self::Item>;
}

#[derive(Debug)]
struct StringList {
    data: Vec<String>
}

impl StringList {
    fn new(init: &[&str]) -> StringList {
        StringList { data: init.iter().map(|s| s.to_string()).collect() }
    }
}

impl Iterator for StringList {
    type Item = String;

    fn next(&self) -> Option<&Self::Item> {
        self.data.iter().next()
    }
}