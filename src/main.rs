use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    // This Funcition takes a number to exeute as UID
    println!("number {} is runing", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}
#[tokio::main(worker_threads = 1)]
async fn main() {
    let now = time::Instant::now();
    let future_one = do_something(1);

    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    let outcome = future_one.await;

    println!("time elaspeld {:?}", now.elapsed());
    println!("Here is the outcome {}", outcome);
}
