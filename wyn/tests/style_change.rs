/*
 *  Crate: Wyn
 *   Test: Style-Change
 */

//! In this test, a window will be created and shown to the user.
//! Every few seconds, the window will change state to test the Styling functionality.
//! Each state-change will be printed to the terminal for reference.
//!
//! The changes will occur in this order, in a loop:
//! [1:NORMAL -> 2:CAPTIONLESS -> 3:BORDERLESS -> 4:FULLSCREEN]
//!
//! When the user is done observing the program, they may close the window. (Right-Click the icon in the taskbar for convenience)
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 4)"]
pub fn style_change() {
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

        while events.is_running() {
            println!();

            {
                self.window.read(|window| {
                    // Restore, just in-case Window is still Fullscreen.
                    window.restore(events).unwrap();
                });
            }

            // STEP 1
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 1]: CAPTIONED");
                    let style = WindowStyle::Captioned;
                    window.set_style(events, style).unwrap();
                    let new_style = window.style(events).unwrap();
                    assert_eq!(style, new_style);
                });
                self.sleep(1.0);
            }

            // STEP 2
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 2]: BORDERED");
                    let style = WindowStyle::Bordered;
                    window.set_style(events, style).unwrap();
                    let new_style = window.style(events).unwrap();
                    assert_eq!(style, new_style);
                });
                self.sleep(1.0);
            }

            // STEP 3
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 3]: BORDERLESS");
                    let style = WindowStyle::Borderless;
                    window.set_style(events, style).unwrap();
                    let new_style = window.style(events).unwrap();
                    assert_eq!(style, new_style);
                });
                self.sleep(1.0);
            }

            // STEP 4
            {
                if !events.is_running() {
                    break;
                }
                self.window.read(|window| {
                    println!("[STEP 4]: FULLSCREEN");
                    window.fullscreen(events).unwrap();
                    assert!(window.is_fullscreen(events));
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
            window.rename(events, "UA-Test M1-S4").unwrap();

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
