use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

pub struct TimedWrapper<Fut: Future> {
	start: Option<Instant>,
	future: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
	pub fn new(future: Fut) -> Self {
		Self { future, start: None }
	}
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
	// This future will output a pair of values:
	// 1. The value from the inner future
	// 2. How long it took for the inner future to resolve
	type Output = (Fut::Output, Duration);

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		// Call the inner poll, measuring how long it took.
		let start = self.start.get_or_insert_with(Instant::now);
		let inner_poll = self.future.poll(cx);
		let elapsed = start.elapsed();

		match inner_poll {
			// The inner future needs more time, so this future needs more time too
			Poll::Pending => Poll::Pending,
			// Success!
			Poll::Ready(output) => Poll::Ready((output, elapsed)),
		}
	}
}

fn main() {
    
}

/* //bo tokio va reqwest di, day la code loi
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};


pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,
    
    future: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
    pub fn new(future: Fut) -> Self {
        Self {
            future,
            start: None,
        }
    }
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        //let mut this = self.project();
        // Call the inner poll, measuring how long it took.
        let start = self.start.get_or_insert_with(Instant::now);
        let inner_poll = self.future.poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            // The inner future needs more time, so this future needs more time too
            Poll::Pending => Poll::Pending,
            // Success!
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}

fn main() {
    
}

*/


/* //code chuan voi tokio va reqwest
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

#[pin_project::pin_project]
pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,
    #[pin]
    future: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
    pub fn new(future: Fut) -> Self {
        Self {
            future,
            start: None,
        }
    }
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();
        // Call the inner poll, measuring how long it took.
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.future.as_mut().poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            // The inner future needs more time, so this future needs more time too
            Poll::Pending => Poll::Pending,
            // Success!
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}

#[tokio::main]
async fn main() {
    let (resp, time) = TimedWrapper::new(reqwest::get("http://adamchalmers.com")).await;
    println!(
        "Got a HTTP {} in {}ms",
        resp.unwrap().status(),
        time.as_millis()
    )
}

*/
