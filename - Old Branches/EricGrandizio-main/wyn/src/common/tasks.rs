/*
 *  Crate: Wyn
 * Module: Common - Tasks
 */

//! Synchronous and Asynchronous types for cross-thread task scheduling.
//!
//! These objects are intended to be created by the `execute` functions in the `event_loop` module.\
//! They can then be consumed by users of the library.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::event_loop::EventLoop;

use core::cell::Cell;
use core::task::Poll;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

// ================================================================================================================================ //

/// A callback function to be executed on the Event Thread.
pub(crate) type Task = Box<dyn FnOnce() + Send + 'static>;

// -------------------------------------------------------------------------------------------------------------------------------- //

/// A polymorphic state that holds either a Synchronous or Asynchonous Future.\
/// Each of the Future types contained within have the same API, allowing the same code to handle both cases.
pub(crate) enum FutureState<T> {
    /// Synchronous Future, used when scheduling tasks from the Event Thread.
    Sync(SyncFuture<T>),
    /// Asynchronous Future, used when scheduling tasks from a thread that is not the Event Thread.
    Async(Arc<AsyncFuture<T>>),
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// A container for a return value that will be received potentially sometime in the future.\
/// When a `Task` is scheduled by calling `EventLoop::execute` or similar functions, this struct is returned.\
/// This will allow users to asynchronously poll or await for the return value of the callback.
pub struct ExecFuture<T> {
    /// The enum holding the type of Future.
    ///
    /// A private inner field is used instead of making `ExecFuture` itself the enum
    /// in order to hide implementation details from users of the library.
    inner: FutureState<T>,
}

impl<T> ExecFuture<T> {
    /// Creates a new `ExecFuture` for Synchronous use.
    pub(crate) fn new_sync(val: Option<T>) -> Self {
        let fut = SyncFuture::new(val);
        let inner = FutureState::Sync(fut);
        Self { inner }
    }

    /// Creates a new `ExecFuture` for Asynchronous use, as well as the
    /// corresponding `AsyncFuture` for the Event Thread to send its data through.
    pub(crate) fn new_async(val: Option<T>) -> (Self, Arc<AsyncFuture<T>>) {
        let fut = AsyncFuture::new(val);

        let arc_recv = Arc::new(fut);
        let arc_send = arc_recv.clone();

        let inner = FutureState::Async(arc_recv);
        (Self { inner }, arc_send)
    }

    /// Stores the value in the Future and notifies any threads waiting for the result.
    #[allow(unused)]
    pub(crate) fn notify(&self, val: T) {
        match &self.inner {
            FutureState::Sync(fut) => fut.notify(val),
            FutureState::Async(fut) => fut.notify(val),
        }
    }

    /// Removes the value from the Future and returns it, if any.
    #[allow(unused)]
    pub(crate) fn take(&self) -> Option<T> {
        match &self.inner {
            FutureState::Sync(fut) => fut.take(),
            FutureState::Async(fut) => fut.take(),
        }
    }

    // ---------------------------------------------------------------- //

    /// Await for the Future to have a stored value ready to be received.
    pub fn wait(&self) -> T {
        match &self.inner {
            FutureState::Sync(fut) => fut.wait(),
            FutureState::Async(fut) => fut.wait(),
        }
    }

    /// Await for the Future to have a stored value ready to be received.\
    /// This function will return without data after the timeout period has elapsed.
    pub fn wait_timeout(&self, timeout: Duration) -> Poll<T> {
        match &self.inner {
            FutureState::Sync(fut) => fut.wait_timeout(timeout),
            FutureState::Async(fut) => fut.wait_timeout(timeout),
        }
    }

    /// Await for the Future to have a stored value ready to be received.\
    /// This function will return without data after the deadline has elapsed.
    pub fn wait_deadline(&self, deadline: Instant) -> Poll<T> {
        match &self.inner {
            FutureState::Sync(fut) => fut.wait_deadline(deadline),
            FutureState::Async(fut) => fut.wait_deadline(deadline),
        }
    }

    /// Poll the Future to retrieve the data inside, if any.
    pub fn poll(&self) -> Poll<T> {
        match &self.inner {
            FutureState::Sync(fut) => fut.poll(),
            FutureState::Async(fut) => fut.poll(),
        }
    }
}

// ================================================================================================================================ //

/// A Future to be used when Tasks are scheduled from the Event Thread,
/// and thus needs no extra synchronization primitives to return the data.
pub(crate) struct SyncFuture<T> {
    /// The data to be stored/received, wrapped in a `Cell` for single-threaded access.
    cell: Cell<Option<T>>,
}

impl<T> SyncFuture<T> {
    /// Creates a new `SyncFuture` holding the specified value, if any.
    pub(crate) const fn new(opt: Option<T>) -> Self {
        let cell = Cell::new(opt);
        Self { cell }
    }

    /// Stores the value in the Future.
    #[allow(unused)]
    pub(crate) fn notify(&self, val: T) {
        self.cell.set(Some(val));
    }

    /// Removes the value from the Future and returns it, if any.
    pub(crate) fn take(&self) -> Option<T> {
        self.cell.take()
    }

    // ---------------------------------------------------------------- //

    /// Await for the Future to have a stored value ready to be received.\
    /// Because this Future is single-threaded, it does not wait for any data.\
    /// If there is no data, this function will `panic` to avoid deadlock.
    pub fn wait(&self) -> T {
        self.take().expect("SyncFuture should be set.")
    }

    /// Await for the Future to have a stored value ready to be received.\
    /// Because this Future is single-threaded, it does not wait for any data.\
    /// If there is no data, this function will return immediately.
    pub fn wait_timeout(&self, _timeout: Duration) -> Poll<T> {
        self.poll()
    }

    /// Await for the Future to have a stored value ready to be received.\
    /// Because this Future is single-threaded, it does not wait for any data.\
    /// If there is no data, this function will return immediately.
    pub fn wait_deadline(&self, _deadline: Instant) -> Poll<T> {
        self.poll()
    }

    /// Poll the Future to retrieve the data inside, if any.
    pub fn poll(&self) -> Poll<T> {
        match self.take() {
            Some(val) => Poll::Ready(val),
            None => Poll::Pending,
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// A Future to be used when Tasks are scheduled from threads other than the Event Thread,
/// and thus needs to use synchronization primitives to return the data safely across threads.
pub(crate) struct AsyncFuture<T> {
    /// The Condition Variable to allow threads to wait and be notified when the data is ready.
    cond: Condvar,

    /// The data to be stored/received, wrapped in a `Mutex` for thread-safe access.
    var: Mutex<Option<T>>,
}

impl<T> AsyncFuture<T> {
    /// Creates a new `AsyncFuture` holding the specified value, if any.
    pub(crate) const fn new(val: Option<T>) -> Self {
        let cond = Condvar::new();
        let var = Mutex::new(val);
        Self { cond, var }
    }

    /// Stores the value in the Future and notifies the thread waiting for the result.
    pub(crate) fn notify(&self, val: T) {
        {
            let mut lock = self.var.lock().unwrap();
            *lock = Some(val);
        }
        self.cond.notify_one();
    }

    /// Removes the value from the Future and returns it, if any.
    #[allow(unused)]
    pub(crate) fn take(&self) -> Option<T> {
        let mut lock = self.var.lock().unwrap();
        lock.take()
    }

    // ---------------------------------------------------------------- //

    /// Await for the Future to have a stored value ready to be received.
    pub fn wait(&self) -> T {
        let lock = self.var.lock().unwrap();

        let res = self.cond.wait_while(lock, |opt| opt.is_none());
        let mut lock = res.unwrap();

        lock.take().expect("AsyncFuture should be set.")
    }

    /// Await for the Future to have a stored value ready to be received.\
    /// This function will return without data after the timeout period has elapsed.
    pub fn wait_timeout(&self, dur: Duration) -> Poll<T> {
        let lock = self.var.lock().unwrap();

        let res = self.cond.wait_timeout_while(lock, dur, |opt| opt.is_none());
        let (mut lock, _timeout) = res.unwrap();

        match lock.take() {
            Some(val) => Poll::Ready(val),
            None => Poll::Pending,
        }
    }

    /// Await for the Future to have a stored value ready to be received.\
    /// This function will return without data after the deadline has elapsed.
    pub fn wait_deadline(&self, deadline: Instant) -> Poll<T> {
        let lock = self.var.lock().unwrap();

        let dur = deadline.saturating_duration_since(Instant::now());
        let res = self.cond.wait_timeout_while(lock, dur, |opt| opt.is_none());
        let (mut lock, _timeout) = res.unwrap();

        match lock.take() {
            Some(val) => Poll::Ready(val),
            None => Poll::Pending,
        }
    }

    /// Poll the Future to retrieve the data inside, if any.
    pub fn poll(&self) -> Poll<T> {
        let mut lock = self.var.lock().unwrap();

        match lock.take() {
            Some(val) => Poll::Ready(val),
            None => Poll::Pending,
        }
    }
}

// ================================================================================================================================ //
