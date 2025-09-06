use std::thread;
fn main() {
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });

        s.spawn(|| {
            // this results error: for n in numbers
            for n in numbers {
                println!("{n}");
            }
        });
    })
}
