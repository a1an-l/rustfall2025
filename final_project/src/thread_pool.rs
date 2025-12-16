use std::sync::{Arc, Mutex, Condvar};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    queue: Arc<(Mutex<Vec<Job>>, Condvar)>,
    shutdown: Arc<Mutex<bool>>,
}

struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let queue = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
        let shutdown = Arc::new(Mutex::new(false));

        let mut workers = Vec::new();

        for id in 0..size {
            workers.push(Worker::new(id, queue.clone(), shutdown.clone()));
        }

        Self {
            workers,
            queue,
            shutdown,
        }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (lock, cvar) = &*self.queue;
        let mut q = lock.lock().unwrap();
        q.push(Box::new(job));
        cvar.notify_one();
    }

    pub fn shutdown(self) {
        {
            let mut s = self.shutdown.lock().unwrap();
            *s = true;
        }

        let (_, cvar) = &*self.queue;
        cvar.notify_all();

        for worker in self.workers {
            if let Some(t) = worker.thread {
                t.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(
        id: usize,
        queue: Arc<(Mutex<Vec<Job>>, Condvar)>,
        shutdown: Arc<Mutex<bool>>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            let job = {
                let (lock, cvar) = &*queue;
                let mut q = lock.lock().unwrap();

                while q.is_empty() {
                    if *shutdown.lock().unwrap() {
                        return;
                    }
                    q = cvar.wait(q).unwrap();
                }

                q.pop()
            };

            if let Some(job) = job {
                job();
            }
        });

        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}
