use std::{thread::{self, JoinHandle}, time::Duration, sync::{mpsc, Mutex, Arc}, cell::RefCell, rc::Rc};

fn main() {
    println!("Hello, world!");
    
    println!("Doing threads....");
//    threads();

    //.........channels
    println!("\n Doing channels....");
//    channels();


    println!("\n Doing mutexes");
let rc: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 1..5 {
        let counter = Arc::clone(&rc);
        let handle: JoinHandle<()> = thread::spawn( move || {
            let mut n = counter.lock().unwrap();
            *n += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {:?}", rc);


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
