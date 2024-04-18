#![doc = include_str!("../README.md")]
#[forbid(unsafe_code)]
#[warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use pin_project_lite::pin_project;

pin_project! {
    /// A future that can be paused and resumed.
    pub struct Pausable<F> {
        #[pin]
        future: F,
        inner: Controller,
    }
}

#[derive(Debug, Clone)]
/// The controller of a `Pausable` future.
pub struct Controller(Arc<Mutex<ControllerInner>>);

#[derive(Debug)]
struct ControllerInner {
    paused: bool,
    cx: Option<Waker>,
}

impl<F: Future> Future for Pausable<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        let mut inner = (*me.inner).0.lock().unwrap_or_else(|e| e.into_inner());
        if !inner.paused { return me.future.poll(cx); }
        let cx = cx.waker().clone();
        inner.cx.replace(cx);
        Poll::Pending
    }
}

impl<F: Future> Pausable<F> {
    /// Create a new `Pausable` future.
    pub fn new(future: F) -> Self {
        Self {
            future,
            inner: Controller(Arc::new(Mutex::new(ControllerInner {
                paused: false,
                cx: None
            }))),
        }
    }

    /// Get the controller.
    pub fn controller(&self) -> Controller {
        self.inner.clone()
    }
}

impl Controller {
    /// Is the future paused?
    pub fn is_paused(&self) -> bool {
        let me = self.0.lock().unwrap_or_else(|e| e.into_inner());
        me.paused
    }

    /// Pause the future.
    pub fn pause(&self) {
        let mut me = self.0.lock().unwrap_or_else(|e| e.into_inner());
        me.paused = true;
    }

    /// Resume the future.
    pub fn resume(&self) {
        let mut me = self.0.lock().unwrap_or_else(|e| e.into_inner());
        me.paused = false;
        if let Some(cx) = me.cx.take() {
            cx.wake();
        }
    }
}
