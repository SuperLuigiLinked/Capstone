/*
 *  Crate: Wyn
 *   Test: Errors
 */

//! This test ensures multiple event loops cannot be started.

mod utils;

// ================================================================================================================================ //

#[test]
pub fn errors_a() {
    utils::timeout::test_deadline(5.0);

    let res1 = std::panic::catch_unwind(test_main_a);
    assert!(res1.is_err(), "Multiple Event Loops should panic.");

    let res2 = std::panic::catch_unwind(test_main_a);
    assert!(res2.is_err(), "Poisoned Event Loops should panic.");

    let _ = format!("{:?}", res1);
    let _ = format!("{:?}", res2.unwrap_err());
}

#[should_panic]
#[test]
pub fn errors_b() {
    utils::timeout::test_deadline(5.0);
    test_main_b();
}

#[test]
pub fn errors_c() {
    utils::timeout::test_deadline(5.0);

    #[cfg(target_os = "windows")]
    {
        let err = WinError::new(1).unwrap();
        let _ = format!("{err}{err:?}");
    }
}

// ================================================================================================================================ //

fn test_main_a() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();
    events.run();
}

fn test_main_b() {
    let app = TestApp::new();
    let events = EventLoop::new(&app).unwrap();

    std::thread::scope(|scope| {
        let res = scope.spawn(|| events.run()).join();
        if let Err(err) = res {
            std::panic::resume_unwind(err);
        }
    });
}

// ================================================================================================================================ //

#[allow(unused_imports)]
use wyn::{errors::*, event_loop::*, events::*, inputs::*, screen::*, types::*, window::*, *};

// -------------------------------------------------------------------------------------------------------------------------------- //

struct TestApp {}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl TestApp {
    pub fn new() -> Self {
        Self {}
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        Window::open(events).unwrap();
    }

    fn window_open(&self, _events: &EventLoop, _handle: WindowHandle) {
        // INTENTIONAL PANIC IN EVENT-HANDLER CALLBACK.
        EventLoop::new(self).unwrap();
    }
}

// ================================================================================================================================ //
