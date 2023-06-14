/*
 *  Crate: Wyn
 *   Test: Rename
 */

//! This example opens a Window, attempts to set the name, and then requests the name back.
//!
//! If the returned string is identical to the string passed in, then the test passes.

mod utils;

// ================================================================================================================================ //

#[test]
pub fn rename() {
    utils::timeout::test_deadline(5.0);
    test_main();
}

// ================================================================================================================================ //

fn test_main() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();
    events.run();

    assert!(app.started.load(Ordering::Relaxed));
    assert!(app.stopped.load(Ordering::Relaxed));
    assert!(app.opened.load(Ordering::Relaxed));
    assert!(app.closed.load(Ordering::Relaxed));
}

// ================================================================================================================================ //

#[allow(unused_imports)]
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, types::*, window::*, *};

use std::sync::atomic::{AtomicBool, Ordering};

// -------------------------------------------------------------------------------------------------------------------------------- //

struct TestApp {
    started: AtomicBool,
    stopped: AtomicBool,
    opened: AtomicBool,
    closed: AtomicBool,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl TestApp {
    pub fn new() -> Self {
        let started = AtomicBool::new(false);
        let stopped = AtomicBool::new(false);
        let opened = AtomicBool::new(false);
        let closed = AtomicBool::new(false);
        Self {
            started,
            stopped,
            opened,
            closed,
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        self.started.store(true, Ordering::Relaxed);

        let window = Window::open(events).unwrap();

        let text = "Hello, World!";
        window.rename(events, text).unwrap();
        let name = window.name(events).unwrap();
        assert_eq!(name, text);

        window.close(events).unwrap();
    }

    fn stop(&self, _events: &EventLoop) {
        self.stopped.store(true, Ordering::Relaxed);
    }

    fn window_open(&self, _events: &EventLoop, _handle: WindowHandle) {
        self.opened.store(true, Ordering::Relaxed);
    }

    fn window_close(&self, events: &EventLoop, _handle: WindowHandle) {
        self.closed.store(true, Ordering::Relaxed);
        events.request_stop();
    }
}

// ================================================================================================================================ //
