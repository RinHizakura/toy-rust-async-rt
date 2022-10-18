/* This is our simple solution to support multiple asynchronous routine. We
 * can achieve by creating each routine as a task which is a futrue, and bind
 * them to the same scheduler which is also a future. So once the scheduler
 * is awaited, it will poll each underlying task and wake any if it is ready. */
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/* Each of the async blocks are treated as a different type. Since all of them implements
 * Future, we have to actually dynamically dispatch them:
 * https://dailydevsblog.com/troubleshoot/resolved-expected-async-block-found-a-different-async-block-146134/
 */

/* We use crate pin_project to simply seperate normal reference
 * and pinned reference. */
pin_project! {
    pub struct Scheduler<T> {
        #[pin]
        tasks: Vec<Pin<Box<dyn Future<Output = T>>>>,
        outputs: Vec<Option<T>>,
    }
}

impl<T> Scheduler<T> {
    pub fn new() -> Self {
        Scheduler {
            tasks: vec![],
            outputs: vec![],
        }
    }

    // TODO: take a deeper look at the lifetime
    pub fn add(&mut self, task: impl Future<Output = T> + 'static) {
        self.tasks.push(Box::pin(task));
        self.outputs.push(None);
    }
}

impl<T> Future for Scheduler<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();
        let len = this.tasks.len();

        let mut ready_task = 0;
        for i in 0..len {
            /* the related task is completed */
            if this.outputs[i].is_some() {
                ready_task += 1;
                continue;
            }

            if let Poll::Ready(out) = this.tasks[i].as_mut().poll(cx) {
                this.outputs[i] = Some(out);
                ready_task += 1;
            }
        }

        if ready_task == len {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
