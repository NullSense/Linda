use log::info;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

trait FnBox {
    /// Takes ownership of self and moves value out of Box<T>
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        // create JoinHandle<()> instance
        let thread = thread::spawn(move || loop {
            // acquire mutex, panic if it's poisoned (thread panicked while holding lock)
            // if all is good, receive Job from channel
            let job = receiver
                .lock()
                .expect("Poisoned thread.")
                .recv()
                .expect("Thread holding send has shut down.");
            // Mutex makes sure only a single thread can make a request for a job

            info!("Worker id: {} received job; executing.", id);

            job.call_box();
        });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        // can't have 0 threads, panic
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // unwrap is ok, because threads don't stop executing
        // as long as the pool exists, so failure can't happen
        self.sender.send(job).unwrap();
    }
}
