use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();

    for _ in 0..1000000 {
        println!("##########")
    }

    println!("{}", now.elapsed().as_secs());
}
