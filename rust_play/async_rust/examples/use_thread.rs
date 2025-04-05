use std::thread;
use std::time::Duration;

fn foo(n: u64) {
    println!("start {n}");
    thread::sleep(Duration::from_secs(1));
    println!("end {n}");
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 1..=10 {
        thread_handles.push(thread::spawn(move || foo(i)));
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
}
