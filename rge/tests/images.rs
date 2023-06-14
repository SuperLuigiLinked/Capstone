/*
 *  Crate: RGE
 *   Test: Images
 */

//! In this test, a window will be opened for the user to see.
//!
//! An image of the Duquesne Logo will be loaded from a file and displayed on the window.
//! The image will be scaled up and down as time passes.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

// ================================================================================================================================ //

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 2, Story 3)"]
pub fn images() {
    test_main();
}

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

use rge::{EngineState, Game, GameEngine, GameEngineSettings, Texture};

#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4, Vertex, VertexUV};

// ================================================================================================================================ //

/// Duquesne Logo.
const DUQ_IMAGE: &[u8] = include_bytes!("../images/DUQ.jpg");

// ================================================================================================================================ //

fn test_main() {
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
    duq_image: Texture,
    renders: usize,
    updates: usize,
}

impl App {
    pub fn new() -> Self {
        let updates = 0;
        let renders = 0;
        let duq_image = Texture::load_bytes(DUQ_IMAGE, None).unwrap();

        Self {
            duq_image,
            renders,
            updates,
        }
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
        //eprintln!("[UPDATE] {_dbg}");

        true
    }

    fn render(&mut self, engine: &mut EngineState) -> bool {
        self.renders += 1;

        let _dbg = self.debug(engine);
        //eprintln!("[RENDER] {_dbg}");
        engine.window.name = _dbg;

        // -------------------------------- //

        let secs = engine.timer.next_seconds();
        let period = 8.0;
        let phase = period - (period - (secs % (2.0 * period))).abs();
        let length = (phase * phase) as glsl::float;

        let uv_l = length.mul_add(-0.5, 0.5);
        let uv_r = length.mul_add(0.5, 0.5);

        // -------------------------------- //

        engine.render.clear();

        if self.renders == 1 {
            engine.update_atlas(&self.duq_image);
        }

        engine.render.uv_triangle_strip(&[
            VertexUV {
                xyzw: vec4!(-1.0, -1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(uv_l, uv_l),
            },
            VertexUV {
                xyzw: vec4!(1.0, -1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(uv_r, uv_l),
            },
            VertexUV {
                xyzw: vec4!(-1.0, 1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(uv_l, uv_r),
            },
            VertexUV {
                xyzw: vec4!(1.0, 1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(uv_r, uv_r),
            },
        ]);

        true
    }
}

// ================================================================================================================================ //
