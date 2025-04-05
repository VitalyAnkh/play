macro_rules! bad_example {
    () => {{
        format!("hello").as_str()
    }};
}

fn main() {
    let s = bad_example!();
}
