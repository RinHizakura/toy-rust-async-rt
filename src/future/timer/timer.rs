use super::reactor::global_reactor;
use anyhow::{anyhow, Result};
use futures::stream::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

// Reference: https://github.com/smol-rs/async-io/blob/master/src/lib.rs
pub struct Timer {
    // the ID for reactor to distinguish each timer
    id: Option<usize>,
    // the happen time of the next timer event
    next_time: Instant,
    // the period of the happen of timer event
    period: Duration,
}

impl Timer {
    pub fn after(duration: Duration) -> Result<Timer> {
        let opt_t = Instant::now().checked_add(duration);

        match opt_t {
            Some(t) => Ok(Timer::at(t)),
            None => Err(anyhow!(
                "The setting time goes out the bounds of the underlying data structure"
            )),
        }
    }

    pub fn at(instant: Instant) -> Timer {
        Timer::interval_at(instant, Duration::MAX)
    }

    pub fn interval_at(next_time: Instant, period: Duration) -> Timer {
        Timer {
            id: None,
            next_time,
            period,
        }
    }
}

impl Future for Timer {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.poll_next(cx) {
            Poll::Ready(Some(when)) => Poll::Ready(when),
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => unreachable!(),
        }
    }
}

impl Stream for Timer {
    type Item = Instant;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if Instant::now() >= this.next_time {
            // if the timer already expired
            todo!();
        } else {
            // if we have to wait until the timer expired
            match this.id {
                Some(_) => panic!("We don't expect to poll a timer with id"),
                None => {
                    let reactor = &mut *global_reactor().unwrap().lock().unwrap();
                    this.id = Some(reactor.insert_timer(this.next_time, cx.waker()));
                }
            }
        }

        Poll::Pending
    }
}
