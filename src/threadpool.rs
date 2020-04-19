use std::thread::{JoinHandle, Builder, ThreadId, spawn};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, Arc};
use std::marker::Send;

type WorkLoad = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPoolConfig {
    pub max_threads: u8,
}

pub struct ThreadPool {
    config: ThreadPoolConfig,
    sender: Sender<WorkLoad>,
    threads: Vec<JoinHandle<()>>
}

impl ThreadPool {
    pub fn new(config: ThreadPoolConfig) -> Self {
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let threads = ThreadPool::create_threads(config.max_threads, receiver);

        ThreadPool {
            config: config,
            threads: threads,
            sender: sender
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    fn create_threads(count: u8, work_load: Arc<Mutex<Receiver<WorkLoad>>>) -> Vec<JoinHandle<()>> {
        let mut threads = Vec::new();

        for _ in 0..count {
            let work_load_clone: Arc<Mutex<Receiver<WorkLoad>>> = Arc::clone(&work_load);

            threads.push(spawn(move || {
                let job = {
                    let lock = work_load_clone.lock().unwrap();
                    lock.recv().unwrap()
                    // Drop lock before executing job
                };
                job();                
            }));
        }
        threads
    }
}