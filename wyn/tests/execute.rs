/*
 *  Crate: Wyn
 *   Test: Execute
 */

//! This test tests Multi-Threading capabilities.
//! The Event Loop will run for 1 second, then another thread will attempt to terminate it.

mod utils;

// ================================================================================================================================ //

#[test]
pub fn execute() {
    utils::timeout::test_deadline(5.0);
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

    assert!(AFLAG_A.load(Ordering::Acquire));
    assert!(AFLAG_B.load(Ordering::Acquire));
    assert!(AFLAG_C.load(Ordering::Acquire));
    assert!(AFLAG_D.load(Ordering::Acquire));

    assert!(SFLAG_A.load(Ordering::Acquire));
    assert!(SFLAG_B.load(Ordering::Acquire));
    assert!(SFLAG_C.load(Ordering::Acquire));
    assert!(SFLAG_D.load(Ordering::Acquire));
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

static AFLAG_A: AtomicBool = AtomicBool::new(false);
static AFLAG_B: AtomicBool = AtomicBool::new(false);
static AFLAG_C: AtomicBool = AtomicBool::new(false);
static AFLAG_D: AtomicBool = AtomicBool::new(false);

static SFLAG_A: AtomicBool = AtomicBool::new(false);
static SFLAG_B: AtomicBool = AtomicBool::new(false);
static SFLAG_C: AtomicBool = AtomicBool::new(false);
static SFLAG_D: AtomicBool = AtomicBool::new(false);

// -------------------------------------------------------------------------------------------------------------------------------- //

impl TestApp {
    pub fn new() -> Self {
        let started = AtomicBool::new(false);
        let stopped = AtomicBool::new(false);
        Self { started, stopped }
    }

    pub fn run(&self, events: &EventLoop) {
        assert!(events.await_startup());

        // Assert the Event Loop is running independently.
        {
            assert!(
                events.is_running(),
                "The EventLoop should be running, but isn't."
            );

            std::thread::sleep(std::time::Duration::from_secs(1));

            assert!(
                events.is_running(),
                "The EventLoop should still be running, but isn't."
            );
        }

        let fut_a = events.execute_discard(|| AFLAG_A.store(true, Ordering::Release));
        let _ = fut_a.poll();

        let fut_b = events.execute(|| AFLAG_B.store(true, Ordering::Release));
        let _ = fut_b.wait_timeout(std::time::Duration::from_secs(1));

        let fut_c = events.execute(|| AFLAG_C.store(true, Ordering::Release));
        let _ = fut_c.wait_deadline(std::time::Instant::now());

        let fut_d = events.execute(|| AFLAG_D.store(true, Ordering::Release));
        fut_d.wait();

        events.request_stop();

        assert!(events.await_termination());
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for TestApp {
    fn start(&self, events: &EventLoop) {
        self.started.store(true, Ordering::Relaxed);

        let fut_a = events.execute_discard(|| SFLAG_A.store(true, Ordering::Release));
        let _ = fut_a.poll();

        let fut_b = events.execute(|| SFLAG_B.store(true, Ordering::Release));
        let _ = fut_b.wait_timeout(std::time::Duration::from_secs(1));

        let fut_c = events.execute(|| SFLAG_C.store(true, Ordering::Release));
        let _ = fut_c.wait_deadline(std::time::Instant::now());

        let fut_d = events.execute(|| SFLAG_D.store(true, Ordering::Release));
        fut_d.wait();
    }

    fn stop(&self, _events: &EventLoop) {
        self.stopped.store(true, Ordering::Relaxed);
    }
}

// ================================================================================================================================ //
