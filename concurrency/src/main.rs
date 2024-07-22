use std::{thread::{self, JoinHandle}, time::Duration, sync::mpsc};

fn main() {
    println!("Hello, world!");
    
    println!("Doing threads....");
    threads();

    //.........channels
    println!("\n Doing channels....");
    channels();


}

fn channels() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx3.clone();

    thread::spawn(move || {
        let message = String::from("Message from another thread original");
        tx.send(message).unwrap();

        tx.send(String::from("another message")).unwrap();
    });

    thread::spawn(move || {
        let message = String::from("Message from another thread thread 2");
        tx2.send(message).unwrap();
    });

    thread::spawn(move || {
        let message = String::from("Message from another thread thread 3");
        tx3.send(message).unwrap();
    });

    thread::spawn(move || {
        let message = String::from("Message from another thread thread 4");
        tx4.send(message).unwrap();
    });

    for recv in rx {
        println!("Recieved in main: {recv}")
    }
}

fn threads() {
    let list = vec![1,2,3,4,5];

    let handle: JoinHandle<()> = thread::spawn(move || {
        println!("List here: {list:?}");
        for i in &list {
            println!("Greetings from thread {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    handle.join().unwrap();
}
