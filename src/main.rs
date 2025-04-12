use std::{thread, time};

fn do_something(number: i8) -> i8 {
    // This Funcition takes a number to exeute as UID
    println!("number {} is runing", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    let now = time::Instant::now();
    let one: i8 = do_something(1);
    let two: i8 = do_something(2);
    let three: i8 = do_something(3);
    println!("time elaspeld {:?}", now.elapsed());
    println!("Result {}", one + two + three)
}
