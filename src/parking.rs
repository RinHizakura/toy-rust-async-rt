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
}

struct Inner {
    lock: Mutex<bool>,
    cvar: Condvar,
}

impl Inner {}
