use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/* The WaitCond future will take the atomically reference of an object and
 * wait until it becomes the expected value. */
pub struct WaitCond<T> {
    var: Arc<RefCell<T>>,
    expected: T,
}

impl<T: std::cmp::PartialEq> Future for WaitCond<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let var = self.var.borrow();
        if *var != self.expected {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

pub fn wait_cond<T>(var: Arc<RefCell<T>>, expected: T) -> WaitCond<T> {
    WaitCond { var, expected }
}
