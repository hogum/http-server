use std::thread;
use std::{error, fmt};

/// Error implementation that handles thread pool creation
pub struct PoolCreationError {}

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {}
        Ok(ThreadPool { threads })
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}
