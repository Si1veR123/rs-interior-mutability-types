extern crate interior_mutability;
use interior_mutability::channels::spawn_channels;
use std::thread;


fn main() {
    let (send, recv) = spawn_channels(5);

    let this_thread_send = send.clone();

    let _thread = thread::spawn(move || {
        for i in 0..100 {
            send.send(i*10);
        }
    });

    this_thread_send.send(1000);

    // should output 0 -> 1000 in multiples of 10. may not be in order. 0 -> 90 is sent from spawned thread. 100 is sent from this thread.
    for _ in 0..101 {
        println!("{}", recv.recv());
    }
}
