use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn progess<T>(v: Vec<T>) {
    let mut i = 0;
    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        expensive_calculation(n);
    }
}

fn expensive_calculation<T>(_n: &T) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];
    progess(v)
}
