use toy_rust_async_rt::executor;
use toy_rust_async_rt::future::yield_now;

fn main() {
    let task_a = async {
        for _ in 0..3 {
            let y = yield_now::yield_now();
            println!("TASK A");
            y.await;
        }
    };
    let task_b = async {
        for _ in 0..3 {
            let y = yield_now::yield_now();
            println!("TASK B");
            y.await;
        }
    };

    executor::block_on(async {
        task_a.await;
        task_b.await;
    })
}
