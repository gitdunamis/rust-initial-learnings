use std::{thread::{self, JoinHandle}, time::Duration};

fn main() {
    println!("Hello, world!");
    
    let list = vec![1,2,3,4,5];
    
    let handle: JoinHandle<()> = thread::spawn(move || {
        println!("List here: {list:?}");
        for i in &list {
            println!("Greetings from thread {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });
    
//    println!("Main finishing...list is still intact: {list:?}");
    
    handle.join().unwrap()
}
