use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool instance.
    ///
    /// max_threads is the maximum number of threads that the server may utilize.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if max_threads is 0.

    pub fn new(max_threads: usize) -> ThreadPool {
        assert!(max_threads > 0);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(max_threads);

        for id in 0..max_threads {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }

    // pub fn build(max_threads: usize) -> Result<ThreadPool, &'static str> {
    //     if max_threads > 0 {
    //         Ok(ThreadPool)
    //     } else {
    //         return Err(
    //             "Pool creation error, expected number of threads larger than zero but recieved 0",
    //         );
    //     }
    // }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();

        println!("handling new thread!");
    }
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            println!("worker {id} got a job!");
            job();
        });
        Worker { id, handle }
    }
}
