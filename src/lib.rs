mod parking;

use crate::parking::Parker;
use std::future::Future;
use std::sync::Arc;
use std::task::{RawWaker, RawWakerVTable, Waker};

// create a Waker from our Parker
fn create_waker(parker: Parker) -> Waker {
    let raw = Arc::into_raw(Arc::new(parker)) as *const ();
    let vtable = &ParkerVtable::VTABLE;
    unsafe { Waker::from_raw(RawWaker::new(raw, vtable)) }
}

// the VTABLE struct is used to transform our Parker as Waker
struct ParkerVtable;
impl ParkerVtable {
    const VTABLE: RawWakerVTable =
        RawWakerVTable::new(Self::clone, Self::wake, Self::wake_by_ref, Self::drop);

    unsafe fn clone(ptr: *const ()) -> RawWaker {
        todo!();
    }

    unsafe fn wake(ptr: *const ()) {
        todo!();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        todo!();
    }

    unsafe fn drop(ptr: *const ()) {
        todo!();
    }
}

// executor
pub fn block_on<T: Default>(future: impl Future<Output = T>) -> T {
    let parker = Parker::new();
    let waker = create_waker(parker);

    // TODO: just for success compiling
    T::default()
}
