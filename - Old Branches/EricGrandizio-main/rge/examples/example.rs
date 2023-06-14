/*
 *   Crate: RGE
 * Example: Example.rs
 */

//! Example RGE Program.

// ================================================================================================================================ //

#![allow(clippy::identity_op)]

// ================================================================================================================================ //

use rge::{EngineState, Game, GameEngine, GameEngineSettings, Texture};

#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4};

#[allow(unused_imports)]
use rand::prelude::*;

// ================================================================================================================================ //

pub fn main() {
    let res = Texture::load("rge/examples/images/DUQ.jpg", None);
    let _tex = res.unwrap();
    dbg!(&_tex.as_slice()[0..16]);

    let mut app = App::new();

    let settings = GameEngineSettings {
        fps: 60.0,
        vsync: true,
        width: 256.0 * 3.0,
        height: 256.0 * 3.0,
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

#[allow(unused)]
fn lerp(a: glsl::float, b: glsl::float, t: glsl::float) -> glsl::float {
    t.mul_add(b - a, a)
}

#[allow(unused)]
fn colerp(a: glsl::vec3, b: glsl::vec3, t: glsl::float) -> glsl::vec3 {
    let r = lerp(a.0, b.0, t);
    let g = lerp(a.1, b.1, t);
    let b = lerp(a.2, b.2, t);
    glsl::vec3(r, g, b)
}

#[allow(unused)]
fn hue(percent: glsl::float) -> glsl::vec3 {
    let norm = percent.fract();
    let phase = (norm * 3.0) as u32 % 3;
    let t = (norm % (1.0 / 3.0)) * 3.0;

    match phase {
        0 => {
            let r = lerp(1.0, 0.0, t);
            let g = lerp(0.0, 1.0, t);
            glsl::vec3(r, g, 0.0)
        }
        1 => {
            let g = lerp(1.0, 0.0, t);
            let b = lerp(0.0, 1.0, t);
            glsl::vec3(0.0, g, b)
        }
        2 => {
            let b = lerp(1.0, 0.0, t);
            let r = lerp(0.0, 1.0, t);
            glsl::vec3(r, 0.0, b)
        }
        _ => unreachable!(),
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

        #[allow(unused)]
        let mut rng = rand::thread_rng();

        engine.render.clear();

        // let r = (self.updates / 2 * 1 % 256) as glsl::float / 256.0;
        // let g = (self.updates / 2 * 2 % 256) as glsl::float / 256.0;
        // let b = (self.updates / 2 * 4 % 256) as glsl::float / 256.0;
        // engine.render.backcolor = vec4!(r, g, b);

        //engine.render.backcolor = rgba!(0.0, 0.0, 0.1);

        engine.render.triangle(&[
            glsl::Vertex {
                xyzw: vec4!(1.0, -1.0),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(-0.5, 0.0),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(1.0, 1.0),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
        ]);

        engine.render.triangle(&[
            glsl::Vertex {
                xyzw: vec4!(-1.0, -1.0),
                rgba: rgba!(1.0, 0.0, 0.0, 1.0),
            },
            glsl::Vertex {
                xyzw: vec4!(0.5, 0.0),
                rgba: rgba!(1.0, 0.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(-1.0, 1.0),
                rgba: rgba!(1.0, 0.0, 0.0, 1.0),
            },
        ]);

        true
    }
}

// ================================================================================================================================ //
