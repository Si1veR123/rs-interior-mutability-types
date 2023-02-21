use std::cell::UnsafeCell;
use std::mem;


pub struct Cell<T> {
    inner: UnsafeCell<T>
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Cell<T> {
        Self { inner: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        self.replace(value);
    }

    pub fn replace(&self, value: T) -> T {
        let mut_ptr = self.inner.get();

        let old: T;
        unsafe {
            // SAFETY: valid ptr as just created and cell doesnt give out mutable references to inner so not being mutated
            old = mem::replace(&mut *mut_ptr, value)
        }
        old
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        unsafe { self.inner.get().read() }
    }
}
