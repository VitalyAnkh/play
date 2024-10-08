use std::io::stdin;

fn main() {
    let mut name = String::new();
    println!("What's your name?");
    stdin().read_line(&mut name).expect("failed to read line");
    let name_trimed = name.trim().to_lowercase();
    println!("Hello, {}!", name_trimed);
}
