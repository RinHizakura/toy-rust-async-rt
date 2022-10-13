mod parking;

use crate::parking::Parker;
use std::future::Future;

pub fn block_on<T: Default>(future: impl Future<Output = T>) -> T {
    let parker = Parker::new();

    // TODO: just for success compiling
    T::default()
}
