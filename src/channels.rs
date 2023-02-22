use crate::mutex::Mutex;
use std::thread;
use std::time::Duration;
use std::sync::Arc;

pub fn spawn_channels<T>(capacity: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    // create a lifo channel    
    let data_vec: Vec<T> = Vec::with_capacity(capacity);
    let arc = Arc::new(Mutex::new(data_vec));

    let sender = ChannelSender { q: arc.clone() };
    let recv = ChannelReceiver { q: arc };

    (sender, recv)
}

#[derive(Clone)]
pub struct ChannelSender<T> {
    q: Arc<Mutex<Vec<T>>>
}

impl<'a, T> ChannelSender<T> {
    pub fn send(&self, data: T) {
        self.q.lock().push(data)
    }
}

#[derive(Clone)]
pub struct ChannelReceiver<T> {
    q: Arc<Mutex<Vec<T>>>,
}

impl<T> ChannelReceiver<T> {
    pub fn try_recv(&self) -> Result<Option<T>, ()> {
        // will return Err(()) if cannot lock, will return Ok(None) if empty, will return Ok(T) if item available
        Ok(self.q.try_lock()?.pop())
    }

    pub fn recv(&self) -> T {
        loop {
            match self.q.lock().pop() {
                Some(d) => return d,
                None => thread::sleep(Duration::from_millis(1))
            }
        }
    }

    pub fn ready(&self) -> bool {
        !self.q.lock().is_empty()
    }
}
