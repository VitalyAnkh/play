use std::thread;

fn main() {
    let numbers: Vec<usize> = vec![];
    // let numbers = Vec::from_iter(0..=100);
    let t = thread::spawn(|| {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("the average is {average}");
}
