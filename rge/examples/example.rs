/*
 *   Crate: RGE
 * Example: Example.rs
 */

//! Example RGE Program.

// ================================================================================================================================ //

#![allow(clippy::identity_op)]

mod utils;

// ================================================================================================================================ //

use rge::{EngineState, Game, GameEngine, GameEngineSettings};

#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4, Vertex, VertexUV};

// ================================================================================================================================ //

pub fn main() {
    let mut app = App::new();

    let settings = GameEngineSettings {
        fps: 60.0,
        vsync: true,
        fullscreen: false,
        width: 256.0 * 3.0,
        height: 256.0 * 3.0,
    };

    let engine = GameEngine::new(&mut app, settings);
    engine.run()
}

// ================================================================================================================================ //

struct App {
    renders: usize,
    updates: usize,
}

impl App {
    pub fn new() -> Self {
        let updates = 0;
        let renders = 0;

        Self { renders, updates }
    }
}

// ================================================================================================================================ //

impl App {
    pub fn debug(&self, engine: &EngineState) -> String {
        let renders = self.renders;
        let updates = self.updates;
        let expected = engine.timer.elapsed_frames() + 1.0;
        let dropped = (expected as isize) - (updates as isize);
        let secs = engine.timer.elapsed_seconds();

        format!("RGE | Renders: {renders} | Updates: {updates} | Expected: {expected:.2} | Dropped: {dropped} | Seconds: {secs:.2}")
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut EngineState) -> bool {
        self.updates += 1;

        let _dbg = self.debug(engine);
        eprintln!("[UPDATE] {_dbg}");

        true
    }

    fn render(&mut self, engine: &mut EngineState) -> bool {
        self.renders += 1;

        let _dbg = self.debug(engine);
        eprintln!("[RENDER] {_dbg}");
        engine.window.name = _dbg;

        engine.render.clear();

        // let r = (self.updates / 2 * 1 % 256) as glsl::float / 256.0;
        // let g = (self.updates / 2 * 2 % 256) as glsl::float / 256.0;
        // let b = (self.updates / 2 * 4 % 256) as glsl::float / 256.0;
        // engine.render.backcolor = vec4!(r, g, b);

        //engine.render.backcolor = rgba!(0.0, 0.0, 0.1);

        engine.render.triangle(&[
            Vertex {
                xyzw: vec4!(1.0, -1.0),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
            Vertex {
                xyzw: vec4!(-0.5, 0.0),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
            Vertex {
                xyzw: vec4!(1.0, 1.0),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
        ]);

        engine.render.triangle(&[
            Vertex {
                xyzw: vec4!(-1.0, -1.0),
                rgba: rgba!(1.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                xyzw: vec4!(0.5, 0.0),
                rgba: rgba!(1.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                xyzw: vec4!(-1.0, 1.0),
                rgba: rgba!(1.0, 0.0, 0.0, 1.0),
            },
        ]);

        true
    }
}

// ================================================================================================================================ //
