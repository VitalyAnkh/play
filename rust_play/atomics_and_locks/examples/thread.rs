use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from main thread! vitalyr");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");

    // let v = vec![1, 2, 3, 4, 5];
    let numbers = Vec::from_iter(0..=1000);
    let average = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    })
    .join()
    .unwrap();
    println!("the average is {average}");
    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
