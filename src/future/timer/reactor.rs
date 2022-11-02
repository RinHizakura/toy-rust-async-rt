use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex, Once};
use std::task::Waker;
use std::thread::Builder;
use std::time::Instant;
use std::collections::BTreeMap;

/* FIXME: To be simple, we just use a big lock on the whole structure.
 * We should lock each member in the structure independently instead
 * for better granularity */
static mut TIMER_REACTOR: Option<Arc<Mutex<Reactor>>> = None;
static TIMER_REACTOR_SET: Once = Once::new();

fn init_timer_thread() {
    /* create a thread which will drive the reactor to
     * wake up the expired timer. */
    Builder::new()
        .name("timer".to_string())
        .spawn(|| main_loop())
        .expect("cannot spawn timer thread");
}

fn main_loop() {
    loop {
        let mut lock = global_reactor().unwrap().try_lock();
        if let Ok(ref reactor) = lock {
            reactor.react();
        }
    }
}

pub fn global_reactor() -> Result<&'static Arc<Mutex<Reactor>>> {
    let mut reactor = Err(anyhow!("Fail to get the global reactor"));

    if TIMER_REACTOR_SET.is_completed() {
        reactor = unsafe {
            TIMER_REACTOR
                .as_ref()
                .ok_or(anyhow!("Fail to get the global reactor"))
        };
    } else {
        TIMER_REACTOR_SET.call_once(|| {
            init_timer_thread();
            reactor =
                Ok(unsafe { TIMER_REACTOR.get_or_insert(Arc::new(Mutex::new(Reactor::new()))) });
        });
    }

    reactor
}

pub struct Reactor {
    next_id: usize,
    timers: BTreeMap<(Instant, usize), Waker>,
}

impl Reactor {
    pub fn new() -> Self {
        Reactor { next_id: 0, timers: BTreeMap::new() }
    }

    pub fn insert_timer(&mut self, next_time: Instant, waker: &Waker) -> usize {
        let id = self.next_id;
        self.timers.insert((next_time, id), waker.clone());
        self.next_id += 1;
        id
    }

    pub fn react(&self) {
        todo!()
    }
}
