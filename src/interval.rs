use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time;
use tokio::time::Interval;

pin_project! {
    pub struct IntervalDelay {
        #[pin]
        interval: Interval,
    }
}

impl IntervalDelay {
    pub fn new(dur: Duration) -> Self {
        Self {
            interval: time::interval(dur),
        }
    }
}

impl Future for IntervalDelay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().interval.poll_tick(cx).map(|_| ())
    }
}
