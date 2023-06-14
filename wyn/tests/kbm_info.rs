/*
 *  Crate: Wyn
 *   Test: Keyboard/Mouse-Info
 */

//! In this test, a window will be created and shown to the user.
//!
//! The user is free to move the mouse around on the window, clicking and releasing as they go.
//! They may also (when the window is focused) press, hold, and release keys on the keyboard.
//! As they do so, the inputs received by the window will be printed to the console.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 6)"]
pub fn kbm_info() {
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
        window.rename(events, "UA-Test M1-S6").unwrap();
        window.focus(events).unwrap();
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

    fn cursor_move(&self, _events: &EventLoop, _handle: WindowHandle, point: Point) {
        println!("[CURSOR MOVE] {point:?}")
    }

    fn scroll_wheel(&self, _events: &EventLoop, _handle: WindowHandle, delta_x: f64, delta_y: f64) {
        let delta = (delta_x, delta_y);
        println!("[SCROLL WHEEL] {delta:?}")
    }

    fn button_press(
        &self,
        _events: &EventLoop,
        _handle: WindowHandle,
        button: MouseButton,
        pressed: bool,
    ) {
        let pstr = if pressed { "PRESSED " } else { "RELEASED" };
        println!("[BUTTON {pstr}] {button:?}")
    }

    fn key_press(
        &self,
        _events: &EventLoop,
        _handle: WindowHandle,
        keycode: KeyCode,
        pressed: bool,
    ) {
        let pstr = if pressed { "PRESSED " } else { "RELEASED" };
        println!("[KEY {pstr}] {keycode:?}")
    }

    fn character_input(&self, _events: &EventLoop, _handle: WindowHandle, character: char) {
        let code = character as u32;
        let text = if character.is_control() {
            char::REPLACEMENT_CHARACTER
        } else {
            character
        };
        println!("[CHARACTER INPUT] '{text}' [0x{code:02X}]");
    }
}

// ================================================================================================================================ //
