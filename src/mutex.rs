use std::cell::{UnsafeCell, RefCell};
use std::ops::Deref;
use std::thread;
use std::time::Duration;


#[derive(Debug)]
enum MutexLockState {
    Unlocked,
    Locked
}

pub struct Mutex<T> {
    data: UnsafeCell<T>,
    lock_state: RefCell<MutexLockState>
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            data: UnsafeCell::new(data),
            lock_state: RefCell::new(MutexLockState::Unlocked)
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, ()> {
        println!("{:?}", &*self.lock_state.borrow());
        while let MutexLockState::Locked = &*self.lock_state.borrow() {
            thread::sleep(Duration::from_millis(100));
        }  

        self.lock_state.replace(MutexLockState::Locked);

        Ok(MutexGuard::new(
            self.data.get(),
            &self
        ))
    }

    pub fn try_lock(&self) -> Result<MutexGuard<T>, ()> {
        let unlocked = match &*self.lock_state.borrow_mut() {
            MutexLockState::Unlocked => true,
            MutexLockState::Locked => false,
        };

        if unlocked {
            self.lock_state.replace(MutexLockState::Locked);
            Result::Ok(MutexGuard::new(
                self.data.get(),
                &self
            ))
        } else {
            Err(())
        }
    }

    fn unlock(&self) {
        self.lock_state.replace(MutexLockState::Unlocked);
    }
}

pub struct MutexGuard<'a, T> {
    data: Option<Box<T>>,
    mutex: &'a Mutex<T>
}

impl<'a, T> MutexGuard<'a, T> {
    fn new(data: *mut T, mutex: &'a Mutex<T>) -> MutexGuard<T> {
        // SAFETY: this is created from mutex which always lives longer than this guard, and the pointer is valid for as long as the mutex
        let data_box = unsafe { Box::from_raw(data) };
        MutexGuard {
            data: Some(data_box),
            mutex
        }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
        // get the box data and leak it (prevents deallocation of inner data)
        let d = self.data.take().unwrap();
        let _ = Box::leak(d);
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().unwrap()
    }
}
