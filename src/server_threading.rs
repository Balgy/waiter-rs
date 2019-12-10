use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Task = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewTask(Task),
    Terminate
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Kindly requesting all workers to shutdown.");
        
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).expect("Failed to send terminate message to worker.");
        }
        
        println!("Shutting all workers down.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Failed to join worker thread with main.");
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
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
        let task = Box::new(f);

        self.sender.send(Message::NewTask(task)).expect("Failed to send new task to worker.");
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().expect("Worker failed to aquire mutex lock.").recv().expect("Worker failed to receive a value from the channel.");

            match message {
                Message::NewTask(task) => {
                    println!("Worker {} got a task; processing...", id);
                    task.call_box();
                },
                Message::Terminate => {
                    println!("Worker {} received shutdown request.", id);

                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
