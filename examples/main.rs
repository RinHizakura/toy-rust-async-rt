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
        "TASK A"
    };
    let b = async {
        for _ in 0..3 {
            let y = yield_now::yield_now();
            println!("TASK B");
            y.await;
        }
        "TASK B"
    };

    let mut sche = Scheduler::new();
    let id_a = sche.add(a).ok().unwrap();
    let id_b = sche.add(b).ok().unwrap();

    executor::block_on(async {
        let outputs = sche.await;
        assert_eq!(outputs[id_a], Some("TASK A"));
        assert_eq!(outputs[id_b], Some("TASK B"));
    });
}
