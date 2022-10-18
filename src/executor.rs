use crate::parking::Parker;
use std::future::Future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// create a Waker from our Parker
fn create_waker(arc_parker: Arc<Parker>) -> Waker {
    let raw = Arc::into_raw(arc_parker) as *const ();
    let vtable = &ParkerVtable::VTABLE;
    unsafe { Waker::from_raw(RawWaker::new(raw, vtable)) }
}

// the VTABLE struct is used to transform our Parker as Waker
struct ParkerVtable;
impl ParkerVtable {
    const VTABLE: RawWakerVTable =
        RawWakerVTable::new(Self::clone, Self::wake, Self::wake_by_ref, Self::drop);

    unsafe fn clone(ptr: *const ()) -> RawWaker {
        /* Increase the reference count by clone.
         * See https://doc.rust-lang.org/std/mem/fn.forget.html. */
        let arc_parker = ManuallyDrop::new(Arc::from_raw(ptr as *const Parker));
        let _ = arc_parker.clone();
        RawWaker::new(ptr, &Self::VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        let arc_parker = Arc::from_raw(ptr as *const Parker);
        arc_parker.unpark();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        /* Wake without consuming the reference */
        let arc_parker = ManuallyDrop::new(Arc::from_raw(ptr as *const Parker));
        arc_parker.unpark();
    }

    unsafe fn drop(ptr: *const ()) {
        /* Decrease the reference count by drop. The destructor of Parker
         * will be executed once the reference count decreased to zero. */
        let arc_parker = Arc::from_raw(ptr as *const Parker);
        drop(arc_parker);
    }
}

// The main executor
pub fn block_on<T: Default>(mut future: impl Future<Output = T>) -> T {
    // pin the future to the stack
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    let arc_parker = Arc::new(Parker::new());
    let waker = create_waker(arc_parker.clone());

    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => arc_parker.park(),
        }
    }
}
