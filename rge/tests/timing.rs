/*
 *  Crate: RGE
 *   Test: Timing
 */

//! In this test, a window will be opened, but the user should direct their attention to the terminal.
//!
//! The program will attempt to run the Game Engine at 2 Frames-per-Second.
//! This means that every 1/2 seconds, the Update callback should be run, followed by the Render callback.
//! Every time the Update/Render callbacks are called, some information will be printed to the console.
//!
//! On each line, you should see:
//! 1. the "Update/Render-count", which represents how many times the respective callback was called previously.
//! 2. the "Elapsed Frames", which represents the amount of time elapsed since the Game Engine began, in frames.
//! 3. the "Elapsed Seconds", which represents the amount of time elapsed since the Game Engine began, in seconds.
//!
//! The Update-count should approximately match the Elapsed Frames, whereas it should be approximately double the Elapsed Seconds.
//! If the user drags or resizes the window, the Render-count may shoot up drastically.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

// ================================================================================================================================ //

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 2, Story 1)"]
pub fn timing() {
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
        fps: 2.0,
        vsync: false,
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
}

impl App {
    pub fn new() -> Self {
        let updates = 0;
        let renders = 0;

        Self { updates, renders }
    }
}

// ================================================================================================================================ //

impl Game for App {
    fn update(&mut self, engine: &mut EngineState) -> bool {
        self.updates += 1;

        let update = self.updates - 1;
        let seconds = engine.timer.elapsed_seconds();
        let expected = engine.timer.elapsed_frames();
        eprintln!("\n[UPDATE] {update:<6} | (Elapsed Frames: {expected:.2}) (Elapsed Seconds: {seconds:.4})",);

        true
    }

    fn render(&mut self, engine: &mut EngineState) -> bool {
        self.renders += 1;

        let render = self.renders - 1;
        let seconds = engine.timer.elapsed_seconds();
        let expected = engine.timer.elapsed_frames();
        eprintln!("[RENDER] {render:<6} | (Elapsed Frames: {expected:.2}) (Elapsed Seconds: {seconds:.4})",);

        true
    }
}

// ================================================================================================================================ //
