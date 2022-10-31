use anyhow::{anyhow, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
use std::task::Waker;
use std::time::Instant;

static mut TIMER_REACTOR: Option<Reactor> = None;
static TIMER_REACTOR_SET: Once = Once::new();

pub fn global_reactor() -> Result<&'static Reactor> {
    let mut reactor = Err(anyhow!("Fail to get the global reactor"));

    if TIMER_REACTOR_SET.is_completed() {
        reactor = unsafe {
            TIMER_REACTOR
                .as_ref()
                .ok_or(anyhow!("Fail to get the global reactor"))
        };
    } else {
        TIMER_REACTOR_SET.call_once(|| {
            reactor = Ok(unsafe { TIMER_REACTOR.get_or_insert(Reactor::new()) });
        });
    }

    reactor
}

pub struct Reactor {
    dummy: usize,
}

impl Reactor {
    pub fn new() -> Self {
        Reactor { dummy: 0 }
    }

    pub fn insert_timer(&self, next_time: Instant, waker: &Waker) -> usize {
        /* Since a static id generator is used to provide timer id,
         * we don't need a mutable reference of self in this function. */
        static ID_GENERATOR: AtomicUsize = AtomicUsize::new(1);
        let id = ID_GENERATOR.fetch_add(1, Ordering::Relaxed);
        id
    }
}
