use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;
use std::cell::UnsafeCell;
use std::sync::LazyLock;

static GLOBAL1: AtomicI32 = AtomicI32::new(42);
static GLOBAL2: Mutex<i32> = Mutex::new(42);

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}
static GLOBAL3: SyncUnsafeCell<i32> = SyncUnsafeCell(UnsafeCell::new(42));

static GLOBAL4: LazyLock<i32> = LazyLock::new(|| 42);

fn main() {
    println!("Global value: {}", GLOBAL1.load(Ordering::SeqCst));
    println!("Global value: {}", *GLOBAL2.lock().unwrap());
    unsafe {
        println!("Global value: {}", *GLOBAL3.0.get());
    }
    println!("Global value: {}", *GLOBAL4);
}