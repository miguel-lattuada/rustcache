use std::marker::Send;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

use crate::shared::SharedMemory;

type WorkLoad = Box<dyn FnOnce(&SharedMemory) + Send + 'static>;

pub struct ThreadPoolConfig {
    pub max_threads: u8,
    pub shared_memory: SharedMemory,
}

pub struct ThreadPool {
    config: ThreadPoolConfig,
    channel: (Sender<WorkLoad>, Receiver<WorkLoad>),
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(config: ThreadPoolConfig) -> Self {
        let (sender, receiver) = channel();
        // let receiver = Arc::new(Mutex::new(receiver));
        // let threads = ThreadPoolConfig.create_threads(config.max_threads, receiver);

        ThreadPool {
            config: config,
            threads: Vec::new(),
            channel: (sender, receiver),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(&SharedMemory) + Send + 'static,
    {
        let job = Box::new(f);
        let (sender, _) = &self.channel;
        // self.sender.send(job).unwrap();
        sender.send(job).unwrap();
    }

    pub fn build(&self) {
        let work_load = Arc::new(Mutex::new(self.channel.1));
        let memory = Arc::new(Mutex::new(self.config.shared_memory));
        self.create_threads(self.config.max_threads, memory, work_load);
    }

    fn create_threads(
        &self,
        count: u8,
        shared_memory: Arc<Mutex<SharedMemory>>,
        work_load: Arc<Mutex<Receiver<WorkLoad>>>,
    ) {
        for _ in 0..count {
            let work_load_clone = Arc::clone(&work_load);
            let shared_memory_clone = Arc::clone(&shared_memory);

            self.threads.push(spawn(move || {
                let job = {
                    let lock = work_load_clone.lock().unwrap();
                    lock.recv().unwrap()
                    // Drop lock before executing job
                };

                let memory = {
                    let lock = shared_memory_clone.lock().unwrap();
                    &*lock
                };

                job(memory);
            }));
        }
    }
}
