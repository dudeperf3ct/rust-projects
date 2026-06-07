use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
};

/* There is so many things to understand
 * Signature of thread spawn: FnOnce, Send, 'static
 * Creating empty thread pool that will run the closure
 * Sending and recieving closures in thread safely approach
 * The channel stores a queue of jobs and reciver removes one job from that queue to execute it
 * sender  --->  [ job 1, job 2, job 3 ]  ---> receiver
 * main thread -> [ job queue ] -> workers
 * Since mpsc, reciever is not clonable? How to share one reciever safely amongst threads?
 * receiver uses Mutex to avoid race condition (mutating reciever at the same time)
 * Arc provides thread-safe shared ownership of the same reciever across threads
 */

/* Design
ThreadPool owns sender
Workers share receiver
execute(...) sends jobs
workers receive jobs
workers call job()
*/

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    worker: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let worker = thread::spawn(move || {
            loop {
                let message = reciver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {id} executing a job");
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {id} is terminating");
                        // exit the loop
                        break;
                    }
                }
            }
        });
        Worker { id, worker }
    }
}

pub struct ThreadPool {
    worker_pool: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The num_threads is the number of threads in the pool.
    /// The workers contains threads and id
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(num_threads: usize) -> Self {
        assert!(
            num_threads > 0,
            "Number of threads should be greater than 0"
        );

        let mut threads: Vec<Worker> = Vec::with_capacity(num_threads);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..num_threads {
            threads.push(Worker::new(i, Arc::clone(&rx)));
        }

        ThreadPool {
            worker_pool: threads,
            sender: tx,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
        // I am not sure how sending the closure automatically guarantees that
        // it will be recieved and ran in the worker implementation
    }
}

// Send terminate signal to all workers
// Wait until all the workers are drained
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        // Handles indefinite loop problem 
        for _ in &self.worker_pool {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        // Handles ownership problem for join (join consumes the handle)
        for thread in self.worker_pool.drain(..) {
            println!("Shutting down worker {}", thread.id);
            thread.worker.join().unwrap();
        }
    }
}

// Does not work as main thread is blocked as worker loop forever
// Join only waits for all threads to finish but does not signal them to stop
// Question: How to make workers stop?
// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         for thread in &mut self.worker_pool.drain(..) {
//             println!("Shutting down worker {}", thread.id);

//             thread.worker.join().unwrap();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "Number of threads should be greater than 0"]
    fn test_zero_threads_panic() {
        ThreadPool::new(0);
    }
}
