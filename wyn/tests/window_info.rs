/*
 *  Crate: Wyn
 *   Test: Window-Info
 */

//! In this test, a window will be created and shown to the user.
//!
//! The user is free to drag and resize the Window, click away and click back, etc...
//! As they do so, the state of the Window will be printed to the console.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.
//!
//! ### Note
//!
//! This test is vulnerable to race conditions:
//!
//! When an Event is received (e.g. `window_focused`) saying a Window is now `Focused`, the test will `assert` `window.is_focused()`.
//! However, it is always possible for the user to quickly click away and unfocus the Window before the `assert` is run.
//! This can cause the test to spuriously fail.
//!
//! Instead of removing all `assert` statements (and leaving the code untested), I have decided to leave this note as a warning.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 3)"]
pub fn window_info() {
    test_main();
}

// ================================================================================================================================ //

fn test_main() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();
    events.run();
}

// ================================================================================================================================ //

#[allow(unused_imports)]
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, window::*, *};

use utils::rwopt::RwOpt;

// -------------------------------------------------------------------------------------------------------------------------------- //

struct TestApp {
    window: RwOpt<Window>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl TestApp {
    pub fn new() -> Self {
        let window = RwOpt::new(None);
        Self { window }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        println!("[START]");

        let window = Window::open(events).unwrap();
        self.window.set(Some(window));

        self.window
            .read(|window| {
                let screen = window.screen();
                let name = screen.name(events);
                println!("Window opened on Screen: \"{name}\"");
            })
            .unwrap();

        self.window
            .read(|window| {
                window.rename(events, "UA-Test M1-S3").unwrap();
                window.focus(events).unwrap();
            })
            .unwrap();
    }

    fn stop(&self, _events: &EventLoop) {
        println!("[STOP]");
    }

    fn window_open(&self, _events: &EventLoop, handle: WindowHandle) {
        println!("[WINDOW OPEN] {handle:?}")
    }

    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {
        println!("[WINDOW CLOSE] {handle:?}");

        if let Some(true) = self.window.read(|window| window.handle() == handle) {
            events.request_stop()
        }
    }

    fn window_reposition(&self, events: &EventLoop, _handle: WindowHandle) {
        println!("[WINDOW REPOSITION]");
        self.window.read(|window| {
            let br = window.border_rect(events).unwrap();
            let cr = window.content_rect(events).unwrap();
            println!("*  BORDER: {br:?}");
            println!("* CONTENT: {cr:?}");

            if window.is_minimized(events) {
                println!("[WINDOW MINIMIZED]");
            }
            if window.is_maximized(events) {
                println!("[WINDOW MAXIMIZED]");
            }
            if window.is_fullscreen(events) {
                println!("[WINDOW FULLSCREENED]");
            }
        });
    }

    fn window_visibility(&self, events: &EventLoop, _handle: WindowHandle, visible: bool) {
        // Assert the opposite visibility, because the state hasn't changed yet.
        if visible {
            println!("[WINDOW SHOWN]");
            self.window.read(|window| assert!(window.is_hidden(events)));
        } else {
            println!("[WINDOW HIDDEN]");
            self.window
                .read(|window| assert!(window.is_visible(events)));
        }
    }

    fn window_focus(&self, events: &EventLoop, _handle: WindowHandle, focused: bool) {
        if focused {
            println!("[WINDOW FOCUSED]");
            self.window
                .read(|window| assert!(window.is_focused(events)));
        } else {
            println!("[WINDOW UNFOCUSED]");
            self.window
                .read(|window| assert!(!window.is_focused(events)));
        }
    }
}

// ================================================================================================================================ //
