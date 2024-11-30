use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    work_channel: std::sync::mpsc::Sender<Job>,
    workers: Vec<std::thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(max_workers: u32) -> Self {
        let mut threads = vec![];
        let (tx, rx) = std::sync::mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..max_workers {
            let rx = rx.clone();
            let thread = std::thread::spawn(move || loop {
                let job = {
                    let receiver = rx.lock().unwrap();
                    match receiver.recv() {
                        Ok(job) => job,
                        Err(_) => break,
                    }
                };
                job();
            });
            threads.push(thread);
        }
        ThreadPool {
            work_channel: tx,
            workers: threads,
        }
    }

    pub fn spawn<F>(&mut self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        println!("I am here, spawned.");
        self.work_channel.send(job).unwrap();
    }

    pub fn join(self) {
        drop(self.work_channel);
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}
