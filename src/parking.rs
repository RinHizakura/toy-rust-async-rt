/* This is a simple implementation for thread parking and unparking.
 * It looks like smol-rs has a more difficault design to handle more complicate
 * concurrency: https://github.com/smol-rs/parking/blob/master/src/lib.rs */
use std::sync::{Arc, Condvar, Mutex};

pub struct Parker {
    inner: Arc<Inner>,
}

impl Parker {
    pub fn new() -> Parker {
        Parker {
            inner: Arc::new(Inner {
                lock: Mutex::new(false),
                cvar: Condvar::new(),
            }),
        }
    }

    pub fn park(&self) {
        self.inner.park();
    }

    pub fn unpark(&self) {
        self.inner.unpark();
    }
}

struct Inner {
    lock: Mutex<bool>,
    cvar: Condvar,
}

impl Inner {
    fn park(&self) {
        let mut resumable = self.lock.lock().unwrap();
        while !*resumable {
            resumable = self.cvar.wait(resumable).unwrap();
        }
        *resumable = false;
    }

    fn unpark(&self) {
        let mut resumable = self.lock.lock().unwrap();
        *resumable = true;
        self.cvar.notify_one();
    }
}
