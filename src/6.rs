use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        // do some work here
    });

    handle.join();
}
