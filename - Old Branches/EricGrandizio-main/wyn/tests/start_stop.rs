/*
 *  Crate: Wyn
 *   Test: Start-Stop
 */

//! This test ensures that `EventLoop`s are able to be started/stopped.

mod utils;

// ================================================================================================================================ //

#[test]
pub fn start_stop() {
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
}

// ================================================================================================================================ //

#[allow(unused_imports)]
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, types::*, window::*, *};

use std::sync::atomic::{AtomicBool, Ordering};

// -------------------------------------------------------------------------------------------------------------------------------- //

struct TestApp {
    started: AtomicBool,
    stopped: AtomicBool,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl TestApp {
    pub fn new() -> Self {
        let started = AtomicBool::new(false);
        let stopped = AtomicBool::new(false);
        Self { started, stopped }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        self.started.store(true, Ordering::Relaxed);
        events.request_stop();
    }

    fn stop(&self, _events: &EventLoop) {
        self.stopped.store(true, Ordering::Relaxed);
    }
}

// ================================================================================================================================ //
