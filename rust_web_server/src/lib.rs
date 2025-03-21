use std::{
    sync::{mpsc::{self, Receiver}, Arc, Mutex}, 
    thread,
};

pub struct ThreadPool {
    // 传递给线程池的闭包会处理连接并不返回任何值，所以 T 将会是单元类型()
    // threads: Vec<thread::JoinHandle<()>>,

    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// struct Job;

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        // let mut threads = Vec::with_capacity(size);

        // for _ in 0..size {

        // }

        // ThreadPool { threads }

        let (sender, receiver) = mpsc::channel();

        // 多个线程间共享所有权并允许线程修改其值，需要使用Arc<Mutex<T>>
        // Arc使得多个worker拥有接收端，而Mutex则确保一次只有一个worker能从接收端得到任务
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    // 将 assert! 改为返回Result
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {

    // }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 调用handle 的join 会阻塞当前线程直到handle 所代表的线程结束
            // 确保新建线程在main 退出前结束运行
            // 从worker中获取 thread 的所有权
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // 以下代码会在新线程中异步执行不会阻塞 Worker { id, thread} 的执行
        let thread = thread::spawn(move || loop {

            // 当前Worker获取和释放锁是短暂的
            // 如果采用while let (if let 和 match)，那么直到相关代码块结束都不会丢弃等号右边的临时值
            // 采用while let (if let 和 match)线程在完成一个任务后，会以最快的速度再次竞争锁。如果任务执行时间很短，这种快速循环可能让某个线程在其他线程有机会获取锁之前，连续多次成功获取锁。
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    // 执行任务，也就是传递进来的闭包
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        // 代码会在新线程启动后立即执行，而不需要等待 loop 循环完成
        // 新线程中的 loop 循环会独立于 Worker::new 方法的执行流程。
        Worker { id, thread: Some(thread)}
    }
}

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         let thread = thread::spawn(move || {
//             while let Ok(job) = receiver.lock().unwrap().recv() {
//                 println!("Worker {id} got a job; executing.");
//                 job();
//             }
//         });
//         Worker { id, thread }
//     }
// }