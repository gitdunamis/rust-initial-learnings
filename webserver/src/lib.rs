use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{self, Receiver, RecvError, Sender};

pub struct ThreadPool {
    size: usize,
    pool: Vec<Worker>,
    work_queue: Option<Sender<Work>>
}

impl ThreadPool {

    /// Create a new ThreadPool.
    /// size: Number of threads in pool
    ///Will panic if size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver): (Sender<Work>, Receiver<Work>) = mpsc::channel();
        let arcmut: Arc<Mutex<Receiver<Work>>> = Arc::new(Mutex::new(receiver));

        let workers = (1..=size).map(|id| Worker::new(id,  arcmut.clone())).collect::<Vec<Worker>>();

        Self {
            size,
            pool: workers,
            work_queue: Some(sender)
        }
    }

    pub fn execute<F>(&mut self, f: F) where F: FnOnce() + Send + 'static {
        let work: Work = Box::new(f);

        self.work_queue.as_ref().unwrap().send(work).unwrap(); //push work to workers
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.work_queue.take());

        for worker in &mut self.pool {
            println!("Shutting down worker: {}", worker.id);
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Work>>>) -> Self {
        Self {
            id,
            thread: Some(thread::spawn(move || {
                loop {
                    println!("Worker {id} waiting");
                    // let work: Work = receiver.lock().unwrap().recv().unwrap();
                    let work: Result<Work, RecvError> = receiver.lock().unwrap().recv();
                    match work {
                       Ok(work)  => {
                           println!("Worker {id} got work job to do...executing");

                           work(); //do work

                           println!("Worker {id} finished executing its work");
                       }
                        Err(_) => {
                            println!("Worker {id} disconnected....shutting down");
                            break; //exit the loop
                        }
                    }
                }
            }))  //in production code use thread::Builder
        }
    }
}

type Work = Box<dyn FnOnce() + Send + 'static>;

