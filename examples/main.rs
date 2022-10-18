use toy_rust_async_rt::executor;
use toy_rust_async_rt::future::scheduler::Scheduler;
use toy_rust_async_rt::future::yield_now;

fn main() {
    let a = async {
        for _ in 0..3 {
            let y = yield_now::yield_now();
            println!("TASK A");
            y.await;
        }
    };
    let b = async {
        for _ in 0..3 {
            let y = yield_now::yield_now();
            println!("TASK B");
            y.await;
        }
    };

    let mut sche = Scheduler::new();
    sche.add(a);
    sche.add(b);

    executor::block_on(async {
        sche.await;
    })
}
