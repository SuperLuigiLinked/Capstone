/*
 *  Crate: Wyn
 *   Test: User-Rename
 */

//! In this test, a window will be created and shown to the user.
//!
//! In the terminal window, the user will be prompted to enter some text.
//! The user may type whatever they want, then press ENTER to change the name of the window.
//! They may repeat this as many times as they like.
//!
//! When the user is done observing the program, they may close the window.
//! The user must then press ENTER in the terminal to shut down the program.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 5)"]
pub fn user_rename() {
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

    pub fn run(&self, events: &EventLoop) {
        assert!(events.await_startup());
        let _defer = defer::defer(|| events.request_stop());

        while events.is_running() {
            println!("Please enter a new title for the Window:");

            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let text = buffer.trim();

            println!("You entered: \"{text}\"");
            println!("Attempting to Rename Window...");

            let is_open = self.window.read_opt(|opt| {
                if let Some(window) = opt {
                    window.rename(events, text).unwrap();
                    let name = window.name(events).unwrap();
                    assert_eq!(name, text);

                    println!("Success!\n");
                    true
                } else {
                    println!("Window was Closed!\n");
                    false
                }
            });

            if !is_open {
                break;
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
            window.rename(events, "UA-Test M1-S5").unwrap();
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
            self.window.set(None);
            events.request_stop()
        }
    }
}

// ================================================================================================================================ //
