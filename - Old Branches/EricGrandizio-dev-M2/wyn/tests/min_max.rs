/*
 *  Crate: Wyn
 *   Test: Min-Max
 */

//! In this test, a window will be created and shown to the user.
//! Every second, the window will change state to test the Minimize/Maximize/Restore functionality.
//! Each state-change will be printed to the terminal for reference.
//!
//! The changes will occur in this order, in a loop:
//! [1:MAXIMIZE -> 2:RESTORE] -> [3:MINIMIZE -> 4:RESTORE] -> [5:MAXIMIZE -> 6:MINIMIZE -> 7:RESTORE -> 8:RESTORE]
//!
//! Every RESTORE state should undo the previous MAXIMIZE/MINIMIZE state.
//! When the user is done observing the program, they may close the window. (Right-Click the icon in the taskbar for convenience)
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 2)"]
pub fn min_max() {
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
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, types::*, window::*, *};

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

    fn sleep(&self, secs: f64) {
        std::thread::sleep(std::time::Duration::from_secs_f64(secs));
    }

    pub fn run(&self, events: &EventLoop) {
        assert!(events.await_startup());
        let _defer = defer::defer(|| events.request_stop());

        println!("[BEGIN]");
        self.sleep(1.0);

        while events.is_running() {
            println!();

            // STEP 1
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 1]: MAXIMIZE");
                    window.maximize(events).unwrap();
                    assert!(window.is_maximized(events));
                });
                self.sleep(1.0);
            }

            // STEP 2
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 2]: RESTORE");
                    window.restore(events).unwrap();
                    assert!(window.is_normal(events));
                });
                self.sleep(1.0);
            }

            // STEP 3
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 3]: MINIMIZE");
                    window.minimize(events).unwrap();
                    assert!(window.is_minimized(events));
                });
                self.sleep(1.0);
            }

            // STEP 4
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 4]: RESTORE");
                    window.restore(events).unwrap();
                    assert!(window.is_normal(events));
                });
                self.sleep(1.0);
            }

            // STEP 5
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 5]: MAXIMIZE");
                    window.maximize(events).unwrap();
                    assert!(window.is_maximized(events));
                });
                self.sleep(1.0);
            }

            // STEP 6
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 6]: MINIMIZE");
                    window.minimize(events).unwrap();
                    assert!(window.is_minimized(events));
                });
                self.sleep(1.0);
            }

            // STEP 7
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 7]: RESTORE");
                    window.restore(events).unwrap();
                    assert!(window.is_maximized(events));
                });
                self.sleep(1.0);
            }

            // STEP 8
            {
                self.window.read(|window| {
                    println!("[STEP 8]: RESTORE");
                    window.restore(events).unwrap();
                    assert!(window.is_normal(events));
                });
                self.sleep(1.0);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        self.started.store(true, Ordering::Relaxed);

        let window = Window::open(events).unwrap();
        self.window.set(Some(window));

        self.window.read(|window| {
            window.rename(events, "UA-Test M1-S2").unwrap();

            assert!(window.is_hidden(events));

            window.show(events).unwrap();
            assert!(window.is_visible(events));

            window.focus(events).unwrap();
            assert!(window.is_focused(events));
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
            self.window.set(None);
            events.request_stop()
        }
    }
}

// ================================================================================================================================ //
