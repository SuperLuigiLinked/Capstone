/*
 *  Crate: Wyn
 *   Test: Execute-Modal
 */

//! Similar to the `Execute` test, this test tests Multi-Threading capabilities.
//! The Event Loop will run for a couple seconds, then another thread will attempt to terminate it.
//!
//! On some platforms (ex: Windows), dragging/resizing a window causes it to enter a Modal Loop.
//! If there are any windows in a modal loop, then OS functions like `PostThreadMessage`
//! (used interally by the `execute` library functions) may fail.
//!
//! To properly run this test, when the Window pops up, the User should start dragging or resizing the window.
//! If the User does not let go, and the window manages to close on-time, then the Test has PASSED.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 1)"]
pub fn execute_modal() {
    utils::timeout::test_deadline(6.5);
    test_main();
}

// ================================================================================================================================ //

fn test_main() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();

    std::thread::scope(|scope| {
        let app_thread = scope.spawn(|| app.run(&events));
        events.run();
        app_thread.join().unwrap();
    });

    assert!(app.started.load(Ordering::Relaxed));
    assert!(app.stopped.load(Ordering::Relaxed));
    assert!(app.opened.load(Ordering::Relaxed));
    assert!(app.closed.load(Ordering::Relaxed));
}

// ================================================================================================================================ //

#[allow(unused_imports)]
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, window::*, *};

use std::sync::atomic::{AtomicBool, Ordering};
use utils::rwopt::RwOpt;

// -------------------------------------------------------------------------------------------------------------------------------- //

struct TestApp {
    window: RwOpt<Window>,
    started: AtomicBool,
    stopped: AtomicBool,
    opened: AtomicBool,
    closed: AtomicBool,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl TestApp {
    pub fn new() -> Self {
        let window = RwOpt::new(None);
        let started = AtomicBool::new(false);
        let stopped = AtomicBool::new(false);
        let opened = AtomicBool::new(false);
        let closed = AtomicBool::new(false);
        Self {
            window,
            started,
            stopped,
            opened,
            closed,
        }
    }

    pub fn run(&self, events: &EventLoop) {
        if !events.await_startup() {
            return;
        }

        // Assert the Event Loop is running independently.
        println!("SHUTDOWN IN...");
        {
            for i in (0..=5).rev() {
                assert!(
                    events.is_running(),
                    "The EventLoop should be running, but isn't."
                );
                println!("{i}...");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

            assert!(
                events.is_running(),
                "The EventLoop should still be running, but isn't."
            );
        }

        println!("ATTEMPTING TO SHUT DOWN...");
        events.request_stop();
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        self.started.store(true, Ordering::Relaxed);

        let window = Window::open(events).unwrap();
        self.window.set(Some(window));

        self.window.read(|window| {
            window.rename(events, "UA-Test M1-S1").unwrap();
            window.focus(events).unwrap();
        });
    }

    fn stop(&self, _events: &EventLoop) {
        self.stopped.store(true, Ordering::Relaxed);
    }

    fn window_open(&self, _events: &EventLoop, _handle: WindowHandle) {
        self.opened.store(true, Ordering::Relaxed);
    }

    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {
        self.closed.store(true, Ordering::Relaxed);

        if let Some(true) = self.window.read(|window| window.handle() == handle) {
            let window = self.window.take().unwrap();
            assert!(window.is_visible(events));

            window.hide(events).unwrap();
            assert!(window.is_hidden(events));

            events.request_stop()
        }
    }
}

// ================================================================================================================================ //
