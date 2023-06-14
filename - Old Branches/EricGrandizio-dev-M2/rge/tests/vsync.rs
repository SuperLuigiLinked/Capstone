/*
 *  Crate: RGE
 *   Test: Vsync
 */

//! In this test, a window will be opened, but the user should direct their attention to the terminal.
//!
//! The program will attempt to run the Game Engine at the Monitor's Refresh-Rate.
//! Every second, the amount of updates that occurred will be printed to the console.
//! The Average FPS over the course of the program will also be printed.
//!
//! The Updates-per-Second should be approximately the Monitor's Refresh-Rate.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

// ================================================================================================================================ //

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 2, Story 2)"]
pub fn vsync() {
    test_main();
}

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

use rge::{EngineState, Game, GameEngine, GameEngineSettings};

// ================================================================================================================================ //

fn test_main() {
    let mut app = App::new();

    let settings = GameEngineSettings {
        fps: 0.0,
        vsync: true,
        fullscreen: false,
        width: 256.0,
        height: 256.0,
    };

    let engine = GameEngine::new(&mut app, settings);
    engine.run()
}

// ================================================================================================================================ //

struct App {
    updates: usize,
    renders: usize,

    this_updates: usize,
    last_updates: usize,
    last_second: f64,
}

impl App {
    pub fn new() -> Self {
        let updates = 0;
        let renders = 0;

        let this_updates = 0;
        let last_updates = 0;
        let last_second = 0.0;

        Self {
            updates,
            renders,
            this_updates,
            last_updates,
            last_second,
        }
    }
}

// ================================================================================================================================ //

impl Game for App {
    fn update(&mut self, engine: &mut EngineState) -> bool {
        let this_seconds = engine.timer.elapsed_seconds();
        let this_second = this_seconds.floor();
        self.updates += 1;

        let _count = self.updates - 1;
        //eprintln!("\n[UPDATE] {_count} | {this_seconds:.8}");

        if this_second == self.last_second {
            self.this_updates += 1;
        } else {
            let this_updates = self.this_updates;
            let average_fps = (self.updates as f64) / this_seconds;

            eprintln!(
                "Second {this_second}: Updated {this_updates} times | Average FPS: {average_fps:.4}",
            );

            self.last_second = this_second;
            self.last_updates = self.this_updates;
            self.this_updates = 1;
        }

        true
    }

    fn render(&mut self, engine: &mut EngineState) -> bool {
        let _this_seconds = engine.timer.elapsed_seconds();
        self.renders += 1;

        let _count = self.renders - 1;
        //eprintln!("[RENDER] {_count} | {_this_seconds:.8}");

        true
    }
}

// ================================================================================================================================ //
