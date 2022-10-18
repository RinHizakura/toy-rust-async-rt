use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/* reference: https://github.com/smol-rs/futures-lite/blob/master/src/future.rs#L295 */
pub struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if !self.0 {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

pub fn yield_now() -> YieldNow {
    YieldNow(false)
}
