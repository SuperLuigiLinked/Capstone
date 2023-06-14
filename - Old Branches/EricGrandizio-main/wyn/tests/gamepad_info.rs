/*
 *  Crate: Wyn
 *   Test: Gamepad-Info
 */

//! In this test, a window will be created and shown to the user.
//!
//! The user should plug in at least one XInput-compatible controller.
//! They should press and release buttons, move the sticks, etc...
//! As they do so, the state of the controllers will be printed to the console.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 1, Story 7)"]
pub fn gamepad_info() {
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

use std::io::Write;
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

        let mut iter = 1usize;
        while events.is_running() {
            {
                let mut stdout = std::io::stdout().lock();

                writeln!(stdout, "v ======== Frame {iter} ======== v\n").unwrap();

                let controllers = XInputController::collect(events).unwrap();

                for (i, opt_con) in controllers.iter().enumerate() {
                    match opt_con {
                        None => {
                            let line1 = format!("[Controller {i}]");
                            let line2 = format!("* DISCONNECTED");
                            writeln!(stdout, "{line1}\n{line2}\n").unwrap();
                        }
                        Some(con) => {
                            let button_flag = |flag: bool| match flag {
                                true => 'O',
                                false => '_',
                            };

                            let flag_ba = button_flag(con.a());
                            let flag_bb = button_flag(con.b());
                            let flag_bx = button_flag(con.x());
                            let flag_by = button_flag(con.y());

                            let flag_du = button_flag(con.dpad_u());
                            let flag_dd = button_flag(con.dpad_d());
                            let flag_dl = button_flag(con.dpad_l());
                            let flag_dr = button_flag(con.dpad_r());

                            let flag_lb = button_flag(con.bumper_l());
                            let flag_rb = button_flag(con.bumper_r());
                            let (analog_lt, _) = con.trigger_l();
                            let (analog_rt, _) = con.trigger_r();

                            let flag_ls = button_flag(con.thumb_l());
                            let flag_rs = button_flag(con.thumb_r());
                            let flag_back = button_flag(con.back());
                            let flag_start = button_flag(con.start());

                            let (lx, ly) = con.stick_l();
                            let (rx, ry) = con.stick_r();

                            let packet = con.packet();

                            let line1 = format!("[Controller {i}] <{packet}>");
                            let line2 = format!("* |  A: {flag_ba} |  B: {flag_bb} |  X: {flag_bx} |  Y: {flag_by} |");
                            let line3 = format!("* |  U: {flag_du} |  D: {flag_dd} |  L: {flag_dl} |  R: {flag_dr} |");
                            let line4 = format!("* | LB: {flag_lb} | RB: {flag_rb} | LS: {flag_ls} | RS: {flag_rs} |");
                            let line5 = format!("* |  BACK: {flag_back}  |  START: {flag_start} | LT: {analog_lt:7.5} | RS: {analog_rt:7.5} |");
                            let line6 = format!("* | Left-Stick: ({lx:8.5}, {ly:8.5}) | Right-Stick: ({rx:8.5}, {ry:8.5})");
                            writeln!(
                                stdout,
                                "{line1}\n{line2}\n{line3}\n{line4}\n{line5}\n{line6}\n"
                            )
                            .unwrap();
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / 6.0));
            iter += 1;
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
            window.rename(events, "UA-Test M1-S7").unwrap();
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
