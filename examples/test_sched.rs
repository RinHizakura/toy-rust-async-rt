use std::cell::RefCell;
use std::sync::Arc;
use toy_rust_async_rt::executor;
use toy_rust_async_rt::future::scheduler::Scheduler;
use toy_rust_async_rt::future::wait_cond;
use toy_rust_async_rt::future::yield_now;

fn main() {
    let cond = Arc::new(RefCell::new(false));
    let cond0 = cond.clone();
    let cond1 = cond.clone();
    let a = async move {
        for _ in 0..3 {
            let y = yield_now::yield_now();
            println!("TASK A");
            y.await;
        }
        let mut cond = cond0.borrow_mut();
        *cond = true;
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
    let wait = async move {
        wait_cond::wait_cond(cond1, true).await;
        println!("TASK WAIT");
        "TASK WAIT"
    };

    let mut sche = Scheduler::new();
    let _ = sche.add(wait).ok().unwrap();
    let id_a = sche.add(a).ok().unwrap();
    let id_b = sche.add(b).ok().unwrap();

    executor::block_on(async {
        let outputs = sche.await;
        assert_eq!(outputs[id_a], Some("TASK A"));
        assert_eq!(outputs[id_b], Some("TASK B"));
    });
}
