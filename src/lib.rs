#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Waker};

use pin_project_lite::pin_project;

pin_project! {
    /// A future or a stream that can be paused and resumed.
    pub struct Pausable<F> {
        #[pin]
        inner: F,
        controller: Controller,
    }
}

impl<I> Pausable<I> {
    /// Create a new `Pausable` future/stream.
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            controller: Controller(Arc::new(Mutex::new(ControllerInner {
                paused: false,
                cx: None
            }))),
        }
    }

    /// Get the controller.
    pub fn controller(&self) -> Controller {
        self.controller.clone()
    }
}

#[derive(Debug, Clone)]
/// The controller of a `Pausable` future/stream.
pub struct Controller(Arc<Mutex<ControllerInner>>);

#[derive(Debug)]
struct ControllerInner {
    paused: bool,
    cx: Option<Waker>,
}

impl Controller {
    fn inner(&self) -> MutexGuard<ControllerInner> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Is the future/stream paused?
    pub fn is_paused(&self) -> bool {
        self.inner().paused
    }

    /// Pause the future/stream.
    pub fn pause(&self) {
        self.inner().paused = true;
    }

    /// Resume the future/stream.
    pub fn resume(&self) {
        let mut me = self.inner();
        me.paused = false;
        if let Some(cx) = me.cx.take() {
            cx.wake();
        }
    }
}

impl<F: Future> Future for Pausable<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        let mut controller = me.controller.inner();
        if !controller.paused {
            drop(controller);
            return me.inner.poll(cx);
        }
        let cx = cx.waker().clone();
        controller.cx.replace(cx);
        Poll::Pending
    }
}

#[cfg(feature = "stream")]
impl<S: futures::Stream> futures::Stream for Pausable<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = self.project();
        let mut controller = me.controller.inner();
        if !controller.paused {
            drop(controller);
            return me.inner.poll_next(cx);
        }
        let cx = cx.waker().clone();
        controller.cx.replace(cx);
        Poll::Pending
    }
}
