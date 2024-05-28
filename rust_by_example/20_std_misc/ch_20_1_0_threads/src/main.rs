// Rust provides a mechanism for spawning native OS threads via the spawn function,
// the argument of this function is a moving closure.

use std::thread;

const NTHREADS: u32 = 10;

fn main() {
    // Make a vector to hold the children which are spawned
    let mut childred = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        childred.push(thread::spawn(move || {
            println!("This is thread number {}", i);
        }))
    }

    for child in childred {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
