use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Once};
use std::task::Waker;
use std::thread::Builder;
use std::time::{Duration, Instant};

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
        if let Ok(ref mut reactor) = lock {
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
        Reactor {
            next_id: 0,
            timers: BTreeMap::new(),
        }
    }

    pub fn insert_timer(&mut self, next_time: Instant, waker: &Waker) -> usize {
        /* give every timer an unique id to distinguish them in the BTreeMap if
         * they have same expired time */
        let id = self.next_id;
        self.timers.insert((next_time, id), waker.clone());
        self.next_id += 1;
        id
    }

    pub fn remove_timer(&mut self, next_time: Instant, id: usize) {
        self.timers.remove(&(next_time, id));
    }

    fn process_timers(&mut self, wakers: &mut Vec<Waker>) {
        let now = Instant::now();

        // split the tree into the expired and non-expired timer
        let after_now = self.timers.split_off(&(now + Duration::from_nanos(1), 0));
        /* The non-expired timer will be leaved in the Reactor struct. The
         * expired timer will be waked. */
        let now_and_before_now = std::mem::replace(&mut self.timers, after_now);

        for (_, waker) in now_and_before_now {
            wakers.push(waker);
        }
    }

    pub fn react(&mut self) {
        let mut wakers = Vec::new();
        self.process_timers(&mut wakers);
        for waker in wakers {
            waker.wake();
        }
    }
}
