/*
 *  Crate: Wyn
 *   Test: TEST_NAME_HERE
 */

//! Example Template for Multi-Threaded Wyn Programs.

mod utils;

// ================================================================================================================================ //

//#[test]
pub fn main() {
    //utils::timeout::test_deadline(1.0);
    test_main()
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

    pub fn run(&self, events: &EventLoop) {
        if !events.await_startup() {
            return;
        }

        for i in 1..=3 {
            if !events.is_running() {
                break;
            }
            eprintln!("[THREAD] {i}");
            std::thread::sleep(std::time::Duration::from_secs_f64(1.0));
        }
        events.request_stop();
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        eprintln!("[START]");

        let window = Window::open(events).unwrap();
        self.window.set(Some(window));

        self.window.read(|window| {
            window.rename(events, "Example (Multi)").unwrap();
            window.focus(events).unwrap();
        });
    }

    fn stop(&self, _events: &EventLoop) {
        eprintln!("[STOP]");
    }

    fn window_open(&self, _events: &EventLoop, handle: WindowHandle) {
        eprintln!("[WINDOW OPEN] <{handle:?}>")
    }

    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {
        eprintln!("[WINDOW CLOSE] <{handle:?}>");

        if let Some(true) = self.window.read(|window| window.handle() == handle) {
            self.window.set(None);
            events.request_stop()
        }
    }
}

// ================================================================================================================================ //
