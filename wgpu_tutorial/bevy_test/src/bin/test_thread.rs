use std::thread;
fn main() {
    let numbers = vec![1, 2, 3, 4];
    // not move ownership of numbers into the new thread
    // but no errors
    thread::spawn(|| {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}
