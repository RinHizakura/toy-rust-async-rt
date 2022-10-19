/* This is our simple solution to support multiple asynchronous routine. We
 * can achieve by creating each routine as a task which is a futrue, and bind
 * them to the same scheduler which is also a future. So once the scheduler
 * is awaited, it will poll each underlying task and wake any if it is ready. */
use anyhow::{anyhow, Result};
use pin_project_lite::pin_project;
use std::collections::VecDeque;
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
        tasks: VecDeque<Pin<Box<dyn Future<Output = T>>>>,
        task_id: usize,
        outputs: VecDeque<Option<T>>,
    }
}

impl<T> Scheduler<T> {
    pub fn new() -> Self {
        Scheduler {
            tasks: VecDeque::new(),
            task_id: 0,
            outputs: VecDeque::new(),
        }
    }

    // TODO: take a deeper look at the lifetime
    pub fn add(&mut self, task: impl Future<Output = T> + 'static) -> Result<usize> {
        if self.task_id == usize::MAX {
            return Err(anyhow!("Too many task to add in the scheduler"));
        }
        self.tasks.push_back(Box::pin(task));
        self.outputs.push_back(None);
        let id = self.task_id;
        self.task_id += 1;
        Ok(id)
    }
}

impl<T> Future for Scheduler<T> {
    type Output = Vec<Option<T>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();
        let len = this.tasks.len();

        let mut ready = true;
        for i in 0..len {
            /* the related task is completed */
            if this.outputs[i].is_none() {
                if let Poll::Ready(out) = this.tasks[i].as_mut().poll(cx) {
                    this.outputs[i] = Some(out);
                } else {
                    ready = false;
                }
            }
        }

        if ready {
            let mut outputs = Vec::new();
            // FIXME: possibly optimization?
            while let (Some(v), Some(_)) = (this.outputs.pop_front(), this.tasks.pop_front()) {
                outputs.push(v);
            }
            Poll::Ready(outputs)
        } else {
            Poll::Pending
        }
    }
}
