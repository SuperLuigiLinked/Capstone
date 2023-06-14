/*
 *  Crate: RGE
 *   Test: Shapes
 */

//! In this test, a window will opened for the user to see.
//!
//! Various kinds of shapes of different colors will be drawn to the window.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

// ================================================================================================================================ //

#![allow(clippy::identity_op)]

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 2, Story 6)"]
pub fn shapes() {
    test_main();
}

// ================================================================================================================================ //

use rge::{EngineState, Game, GameEngine, GameEngineSettings};

#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4};

// ================================================================================================================================ //

pub fn test_main() {
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

        let secs = engine.timer.elapsed_seconds() as glsl::float;

        // ---------------------------------------------------------------- //

        engine.render.clear();

        self.render_background(engine, secs);
        self.render_point_circle(engine, secs);
        self.render_diagonal_line(engine);
        self.render_gradient_x(engine);
        self.render_triangle_rgb(engine);
        self.render_triangle_cmy(engine);
        self.render_triangle_strip_rgb(engine);
        self.render_triangle_strip_cmy(engine);
        self.render_color_wheel(engine, secs);
        self.render_yellow_arc(engine, secs);
        self.render_transparent_trapezoid(engine);

        // ---------------------------------------------------------------- //

        true
    }
}

// ================================================================================================================================ //

impl App {
    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Creates a Debug String representing the App's framerate state.
    fn debug(&self, engine: &EngineState) -> String {
        let renders = self.renders;
        let updates = self.updates;
        let expected = engine.timer.elapsed_frames() + 1.0;
        let dropped = (expected as isize) - (updates as isize);
        let secs = engine.timer.elapsed_seconds();

        format!("RGE | Renders: {renders} | Updates: {updates} | Expected: {expected:.2} | Dropped: {dropped} | Seconds: {secs:.2}")
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_background(&mut self, engine: &mut EngineState, secs: glsl::float) {
        let pc = (secs / 8.0) as glsl::float;
        let hue = utils::math::hue(pc);
        let clr = rgba!(1.0, 1.0, 1.0);
        engine.render.backcolor = utils::math::colerp(hue, clr, 0.25);

        let alpha = secs.sin().mul_add(0.5, 0.5) as glsl::float;
        engine.render.backcolor.3 = alpha;
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_point_circle(&mut self, engine: &mut EngineState, secs: glsl::float) {
        let origin = glsl::vec2(-0.9, -0.9);
        let radius = 0.0625;

        let p0 = secs as glsl::float + (0.0 / 3.0);
        let p1 = secs as glsl::float + (1.0 / 3.0);
        let p2 = secs as glsl::float + (2.0 / 3.0);

        let r0 = p0 * glsl::float_consts::TAU;
        let r1 = p1 * glsl::float_consts::TAU;
        let r2 = p2 * glsl::float_consts::TAU;

        let (oy0, ox0) = r0.sin_cos();
        let (oy1, ox1) = r1.sin_cos();
        let (oy2, ox2) = r2.sin_cos();

        let v0 = vec4!(
            ox0.mul_add(radius, origin.0),
            oy0.mul_add(radius, origin.1),
            0.0,
            3.0
        );
        let v1 = vec4!(
            ox1.mul_add(radius, origin.0),
            oy1.mul_add(radius, origin.1),
            0.0,
            2.0
        );
        let v2 = vec4!(
            ox2.mul_add(radius, origin.0),
            oy2.mul_add(radius, origin.1),
            0.0,
            1.0
        );

        // Render Background-Square
        {
            let gray = rgba!(0.25, 0.25, 0.25);
            let bradius = radius * 1.125;

            engine.render.triangle_strip(&[
                glsl::Vertex {
                    xyzw: vec4!(origin.0 - bradius, origin.1 - bradius),
                    rgba: gray,
                },
                glsl::Vertex {
                    xyzw: vec4!(origin.0 + bradius, origin.1 - bradius),
                    rgba: gray,
                },
                glsl::Vertex {
                    xyzw: vec4!(origin.0 - bradius, origin.1 + bradius),
                    rgba: gray,
                },
                glsl::Vertex {
                    xyzw: vec4!(origin.0 + bradius, origin.1 + bradius),
                    rgba: gray,
                },
            ])
        }

        // Render Points
        {
            engine.render.point(glsl::Vertex {
                xyzw: v0,
                rgba: rgba!(1.0, 0.0, 0.0),
            });
            engine.render.point(glsl::Vertex {
                xyzw: v1,
                rgba: rgba!(0.0, 1.0, 0.0),
            });
            engine.render.point(glsl::Vertex {
                xyzw: v2,
                rgba: rgba!(0.0, 0.0, 1.0),
            });
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_diagonal_line(&mut self, engine: &mut EngineState) {
        engine.render.line(&[
            glsl::Vertex {
                xyzw: vec4!(-0.5, -0.5),
                rgba: rgba!(0.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(-0.375, 0.5),
                rgba: rgba!(1.0, 1.0, 1.0),
            },
        ]);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_gradient_x(&mut self, engine: &mut EngineState) {
        let origin = vec4!(-0.75, 0.75);
        let radius = 0.125;
        let v0 = vec4!(origin.0 - radius, origin.1 - radius);
        let v1 = vec4!(origin.0 + radius, origin.1 - radius);
        let v2 = vec4!(origin.0 - radius, origin.1 + radius);
        let v3 = vec4!(origin.0 + radius, origin.1 + radius);

        engine.render.line(&[
            glsl::Vertex {
                xyzw: origin,
                rgba: rgba!(0.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: v0,
                rgba: rgba!(1.0, 0.0, 0.0),
            },
        ]);
        engine.render.line(&[
            glsl::Vertex {
                xyzw: origin,
                rgba: rgba!(0.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: v1,
                rgba: rgba!(1.0, 0.0, 0.0),
            },
        ]);
        engine.render.line(&[
            glsl::Vertex {
                xyzw: origin,
                rgba: rgba!(0.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: v2,
                rgba: rgba!(1.0, 0.0, 0.0),
            },
        ]);
        engine.render.line(&[
            glsl::Vertex {
                xyzw: origin,
                rgba: rgba!(0.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: v3,
                rgba: rgba!(1.0, 0.0, 0.0),
            },
        ]);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_triangle_rgb(&mut self, engine: &mut EngineState) {
        // Triangle
        {
            engine.render.triangle(&[
                glsl::Vertex {
                    xyzw: vec4!(0.0, -0.25),
                    rgba: rgba!(1.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: vec4!(-0.25, 0.25),
                    rgba: rgba!(0.0, 1.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: vec4!(0.25, 0.25),
                    rgba: rgba!(0.0, 0.0, 1.0),
                },
            ]);
        }

        // Outline
        {
            engine.render.line_strip(&[
                glsl::Vertex {
                    xyzw: vec4!(0.0, -0.25),
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: vec4!(-0.25, 0.25),
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: vec4!(0.25, 0.25),
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: vec4!(0.0, -0.25),
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
            ]);
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_triangle_cmy(&mut self, engine: &mut EngineState) {
        engine.render.triangle(&[
            glsl::Vertex {
                xyzw: vec4!(0.0, -0.25),
                rgba: rgba!(1.0, 0.0, 1.0),
            },
            glsl::Vertex {
                xyzw: vec4!(-0.25, -0.75),
                rgba: rgba!(1.0, 1.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(0.25, -0.75),
                rgba: rgba!(0.0, 1.0, 1.0),
            },
        ]);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_triangle_strip_rgb(&mut self, engine: &mut EngineState) {
        engine.render.triangle_strip(&[
            glsl::Vertex {
                xyzw: vec4!(0.50, -0.875),
                rgba: rgba!(1.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(0.75, -0.750),
                rgba: rgba!(1.0, 0.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(0.50, -0.625),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(0.75, -0.500),
                rgba: rgba!(0.0, 1.0, 0.0),
            },
            glsl::Vertex {
                xyzw: vec4!(0.50, -0.375),
                rgba: rgba!(0.0, 0.0, 1.0),
            },
            glsl::Vertex {
                xyzw: vec4!(0.75, -0.250),
                rgba: rgba!(0.0, 0.0, 1.0),
            },
        ]);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_triangle_strip_cmy(&mut self, engine: &mut EngineState) {
        let v0 = vec4!(0.50, -0.250);
        let v1 = vec4!(0.75, -0.125);
        let v2 = vec4!(0.50, 0.000);
        let v3 = vec4!(0.75, 0.125);
        let v4 = vec4!(0.50, 0.250);
        let v5 = vec4!(0.75, 0.375);

        // Triangle Strip
        {
            engine.render.triangle_strip(&[
                glsl::Vertex {
                    xyzw: v0,
                    rgba: rgba!(1.0, 0.0, 1.0),
                },
                glsl::Vertex {
                    xyzw: v1,
                    rgba: rgba!(1.0, 0.0, 1.0),
                },
                glsl::Vertex {
                    xyzw: v2,
                    rgba: rgba!(1.0, 1.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: v3,
                    rgba: rgba!(1.0, 1.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: v4,
                    rgba: rgba!(0.0, 1.0, 1.0),
                },
                glsl::Vertex {
                    xyzw: v5,
                    rgba: rgba!(0.0, 1.0, 1.0),
                },
            ]);
        }

        // Line Strip
        {
            engine.render.line_strip(&[
                glsl::Vertex {
                    xyzw: v0,
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: v1,
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: v2,
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: v3,
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: v4,
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
                glsl::Vertex {
                    xyzw: v5,
                    rgba: rgba!(0.0, 0.0, 0.0),
                },
            ]);
        }

        // Points
        {
            let pt_rgba = rgba!(1.0, 1.0, 1.0);
            let pt_size = 5.0;

            engine.render.point(glsl::Vertex {
                xyzw: vec4!(v0.0, v0.1, 0.0, pt_size),
                rgba: pt_rgba,
            });
            engine.render.point(glsl::Vertex {
                xyzw: vec4!(v1.0, v1.1, 0.0, pt_size),
                rgba: pt_rgba,
            });
            engine.render.point(glsl::Vertex {
                xyzw: vec4!(v2.0, v2.1, 0.0, pt_size),
                rgba: pt_rgba,
            });
            engine.render.point(glsl::Vertex {
                xyzw: vec4!(v3.0, v3.1, 0.0, pt_size),
                rgba: pt_rgba,
            });
            engine.render.point(glsl::Vertex {
                xyzw: vec4!(v4.0, v4.1, 0.0, pt_size),
                rgba: pt_rgba,
            });
            engine.render.point(glsl::Vertex {
                xyzw: vec4!(v5.0, v5.1, 0.0, pt_size),
                rgba: pt_rgba,
            });
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_color_wheel(&mut self, engine: &mut EngineState, secs: glsl::float) {
        let origin = vec4!(0.5, 0.625);
        let radius = 0.25;

        let rad = secs as glsl::float / 4.0 * glsl::float_consts::TAU;
        let half_range = 30.0;

        let num_pt = rad.sin().mul_add(half_range, half_range) as usize + 4;

        let mut fan = Vec::with_capacity(num_pt);
        fan.push(glsl::Vertex {
            xyzw: origin,
            rgba: rgba!(1.0, 1.0, 1.0),
        });

        for i in 0..=num_pt {
            let pc = (i as glsl::float) / (num_pt as glsl::float);
            let rad = pc * glsl::float_consts::TAU;
            let (oy, ox) = rad.sin_cos();

            let xyzw = vec4!(ox.mul_add(radius, origin.0), oy.mul_add(radius, origin.1));
            let rgba = utils::math::hue(pc);

            fan.push(glsl::Vertex { xyzw, rgba });
        }

        engine.render.triangle_fan(&fan);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_yellow_arc(&mut self, engine: &mut EngineState, secs: glsl::float) {
        let origin = vec4!(-0.75, 0.0);
        let radius = 0.125;

        let num_pt = 640;

        let mut fan = Vec::with_capacity(num_pt);
        fan.push(glsl::Vertex {
            xyzw: origin,
            rgba: rgba!(1.0, 1.0, 0.0),
        });

        let limit = (secs * glsl::float_consts::PI).sin().mul_add(0.25, 0.75);

        for i in 0..=num_pt {
            let pc = (i as glsl::float) / (num_pt as glsl::float);

            let rpc = pc.mul_add(2.0, -1.0).abs();
            if rpc > limit {
                continue;
            }

            let rad = pc * glsl::float_consts::TAU;
            let (oy, ox) = rad.sin_cos();

            let xyzw = vec4!(ox.mul_add(radius, origin.0), oy.mul_add(radius, origin.1));
            let rgba = rgba!(1.0, 1.0, 0.0);

            fan.push(glsl::Vertex { xyzw, rgba });
        }

        engine.render.triangle_fan(&fan);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    fn render_transparent_trapezoid(&mut self, engine: &mut EngineState) {
        engine.render.triangle_fan(&[
            glsl::Vertex {
                xyzw: vec4!(-0.250, 0.875),
                rgba: rgba!(1.0, 1.0, 1.0, 0.5),
            },
            glsl::Vertex {
                xyzw: vec4!(-0.500, 0.875),
                rgba: rgba!(1.0, 1.0, 1.0, 0.75),
            },
            glsl::Vertex {
                xyzw: vec4!(-0.375, 0.625),
                rgba: rgba!(1.0, 1.0, 1.0, 0.0625),
            },
            glsl::Vertex {
                xyzw: vec4!(0.000, 0.625),
                rgba: rgba!(1.0, 1.0, 1.0, 0.0625),
            },
            glsl::Vertex {
                xyzw: vec4!(0.125, 0.875),
                rgba: rgba!(1.0, 1.0, 1.0, 0.75),
            },
        ]);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //
}

// ================================================================================================================================ //
