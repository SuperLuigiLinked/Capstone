/*
 *   Crate: Wyn
 * Example: Template-Single
 */

//! Example Template for Single-Threaded Wyn Programs.

mod utils;

// ================================================================================================================================ //

pub fn main() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();
    events.run();
}

// ================================================================================================================================ //

#[allow(unused_imports)]
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, types::*, window::*, *};

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
        eprintln!("[START]");

        let window = Window::open(events).unwrap();
        self.window.set(Some(window));

        self.window.read(|window| {
            window.rename(events, "Example (Single)").unwrap();
            window.focus(events).unwrap();
        });
    }

    fn stop(&self, _events: &EventLoop) {
        eprintln!("[STOP]");
        self.window.set(None);
    }

    fn window_open(&self, _events: &EventLoop, handle: WindowHandle) {
        eprintln!("[WINDOW OPEN] {handle:?}")
    }

    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {
        eprintln!("[WINDOW CLOSE] {handle:?}");

        if let Some(true) = self.window.read(|window| window.handle() == handle) {
            self.window.set(None);
            events.request_stop()
        }
    }
}

// ================================================================================================================================ //
