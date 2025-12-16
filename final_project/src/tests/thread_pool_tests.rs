use crate::thread_pool::ThreadPool;
use std::sync::{Arc, Mutex};

#[test]
fn executes_all_tasks() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..20 {
        let c = counter.clone();
        pool.execute(move || {
            *c.lock().unwrap() += 1;
        });
    }

    pool.shutdown();
    assert_eq!(*counter.lock().unwrap(), 20);
}
