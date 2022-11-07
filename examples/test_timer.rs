use std::time::Duration;
use toy_rust_async_rt::executor;
use toy_rust_async_rt::future::timer::Timer;

fn main() {
    executor::block_on(async {
        Timer::after(Duration::from_secs(5)).unwrap().await;
    });
}
