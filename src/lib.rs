use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::{error, fmt};

/// Error implementation that handles thread pool creation
pub struct PoolCreationError {}

/// Holds jobs and thread workers to execute the jobs
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<JobMessage>,
}

/// Handles sending of closures to threads for execution
struct Worker {
    id: usize,
    join_handle: Option<thread::JoinHandle<()>>,
}

/// Helps take ownership of a value in Box<T>
/// using Box<Self>
trait FnBox {
    fn unwrap_box(self: Box<Self>);
}

enum JobMessage {
    NewJob(Job),
    Terminate,
}

/// Defines FnBox for a type that implements FnOnce()
impl<F: FnOnce()> FnBox for F {
    fn unwrap_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Returns a Pool of threads instance
    ///
    /// # Arguments
    /// size: usize
    /// - Number of threads to have in the pool
    ///
    /// # Panics
    /// - If the size is 0
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError {});
        }

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(JobMessage::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &mut self.workers {
            self.sender.send(JobMessage::Terminate).unwrap();
        }
        println!("Terminating workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.join_handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    /// Creates a worker instance that holds a
    /// thread spawned with an empty closure
    ///
    /// # Arguments
    /// - id: usize
    /// The ID to identify the worker instance
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<JobMessage>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Unable to capture lock")
                .recv()
                .unwrap();
            match message {
                JobMessage::NewJob(job) => {
                    println!("Worker {} received job. Executing...", id);
                    job.unwrap_box();
                }
                JobMessage::Terminate => {
                    println!("Terminating worker {}", id);
                    break;
                }
            }
        });

        Worker {
            id,
            join_handle: Some(thread),
        }
    }
}

impl error::Error for PoolCreationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oops! Invalid thread size. We can't make threads `0`")
    }
}

impl fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{file: {}, line: {} }}", file!(), line!())
    }
}
