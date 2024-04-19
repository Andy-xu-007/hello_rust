use std::{rc::Rc, sync::{mpsc, Arc, Mutex}, thread, time::Duration};

fn main() {
    // 互斥器Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 如果另一个线程拥有锁，并且那个线程panic 了，则lock 调用会失败
            // 所以这里选择unwrap 并在遇到这种情况时使线程panic
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for ele in handles {
        ele.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
