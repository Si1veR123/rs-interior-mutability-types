pub mod mutex;
use mutex::Mutex;

fn main() {
    let mutex = Mutex::new(0u32);

    {
        let lock = mutex.lock().unwrap();

        // should be false as lock is already created
        let new_lock = mutex.try_lock();
        println!("{}", new_lock.is_ok());

        let val = *lock + 3;
        println!("{}", val);
    }

    // should be true as previous lock has been dropped
    let new_lock = mutex.try_lock();
    println!("{}", new_lock.is_ok());

    let new_lock_block = mutex.lock();
    println!("should never output this");
}
