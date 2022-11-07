use std::time::Duration;
use std::time::Instant;
use toy_rust_async_rt::executor;
use toy_rust_async_rt::future::timer::Timer;

fn main() {
    executor::block_on(async {
        let time_a = Instant::now();
        println!("{:?}", time_a);
        Timer::after(Duration::from_secs(5)).unwrap().await;
        let time_b = Instant::now();
        println!("{:?}", time_b);
    });
}
