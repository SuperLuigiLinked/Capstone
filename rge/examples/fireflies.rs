/*
 *   Crate: RGE
 * Example: Fireflies.rs
 */

//! Example RGE Program.

// ================================================================================================================================ //

#![allow(clippy::identity_op)]

mod utils;

// ================================================================================================================================ //

use rand::Rng;
use rge::{EngineState, Game, GameEngine, GameEngineSettings};

#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4, Vertex, VertexUV};

// ================================================================================================================================ //

pub fn main() {
    let mut app = App::new();

    let settings = GameEngineSettings {
        fps: 0.0,
        vsync: true,
        fullscreen: true,
        width: 640.0 * 2.0,
        height: 360.0 * 2.0,
    };

    let engine = GameEngine::new(&mut app, settings);
    engine.run()
}

// ================================================================================================================================ //

struct App {
    renders: usize,
    updates: usize,
    particles: Vec<Particle>,
}

impl App {
    pub fn new() -> Self {
        let updates = 0;
        let renders = 0;
        let particles = Vec::new();

        Self {
            renders,
            updates,
            particles,
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Default)]
struct Particle {
    pub center_x: glsl::float,
    pub center_y: glsl::float,
    //pub radius: glsl::float,
    pub radius_c: glsl::float,
    pub radius_s: glsl::float,
    //pub angle: glsl::float,
    pub angle_c: glsl::float,
    pub angle_s: glsl::float,
    //pub speed: glsl::float,
    pub speed_c: glsl::float,
    pub speed_s: glsl::float,
    pub size: glsl::float,
    pub color: glsl::vec4,
}

impl Particle {
    pub fn position(&self, secs: glsl::float) -> glsl::vec2 {
        // let angle = secs.mul_add(self.speed, self.angle);
        // let (sin, cos) = angle.sin_cos();

        let angle_c = secs.mul_add(self.speed_c, self.angle_c);
        let cos = angle_c.cos();

        let angle_s = secs.mul_add(self.speed_s, self.angle_s);
        let sin = angle_s.sin();

        // let x = cos.mul_add(self.radius, self.center_x);
        // let y = sin.mul_add(self.radius, self.center_y);

        let x = cos.mul_add(self.radius_c, self.center_x);
        let y = sin.mul_add(self.radius_s, self.center_y);

        glsl::vec2(x, y)
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

    fn generate_particles(&mut self) {
        let mut rng = rand::thread_rng();

        for _idx in 0..20_000 {
            let center_x = rng.gen_range(-1.0..=1.0);
            let center_y = rng.gen_range(-1.0..=1.0);
            //let radius = rng.gen_range(0.0..=0.05);
            let radius_c = rng.gen_range(0.0..=0.1);
            let radius_s = rng.gen_range(0.0..=0.1);
            //let angle = rng.gen_range(0.0..glsl::float_consts::TAU);
            let angle_c = rng.gen_range(0.0..glsl::float_consts::TAU);
            let angle_s = rng.gen_range(0.0..glsl::float_consts::TAU);
            //let sign = (rng.gen::<bool>() as i32 * 2 - 1) as glsl::float;
            let sign_c = (rng.gen::<bool>() as i32 * 2 - 1) as glsl::float;
            let sign_s = (rng.gen::<bool>() as i32 * 2 - 1) as glsl::float;
            //let speed = rng.gen_range(0.0125..(glsl::float_consts::TAU / 3.0)) * sign;
            let speed_c = rng.gen_range(0.0125..(glsl::float_consts::TAU / 3.0)) * sign_c;
            let speed_s = rng.gen_range(0.0125..(glsl::float_consts::TAU / 3.0)) * sign_s;
            let size = (rng.gen_range(1..=4) as glsl::float).floor();

            let color_r = rng.gen_range(0.0..=1.0);
            let color_g = rng.gen_range(0.5..=1.0);
            let color_b = 0.0; //rng.gen_range(0.0..=1.0);
            let color_a = 1.0; //rng.gen_range(0.25..=1.0);
            let color = glsl::vec4(color_r, color_g, color_b, color_a);

            // let color = match _idx % 5 {
            //     0 => glsl::vec4(1.0, 0.0, 0.0, 1.0),
            //     1 => glsl::vec4(0.0, 1.0, 0.0, 1.0),
            //     2 => glsl::vec4(0.1, 0.2, 1.0, 1.0),
            //     3 => glsl::vec4(1.0, 0.9, 0.0, 1.0),
            //     _ => glsl::vec4(1.0, 1.0, 1.0, 1.0),
            // };

            //let color = glsl::vec4(1.0, 1.0, 1.0, color_a);

            let particle = Particle {
                center_x,
                center_y,
                //radius,
                radius_c,
                radius_s,
                //angle,
                angle_c,
                angle_s,
                //speed,
                speed_c,
                speed_s,
                size,
                color,
            };

            self.particles.push(particle);
        }
    }

    fn update_particles(&mut self) {}
}

impl Game for App {
    fn update(&mut self, engine: &mut EngineState) -> bool {
        self.updates += 1;

        let _dbg = self.debug(engine);
        //eprintln!("[UPDATE] {_dbg}");

        if self.updates == 1 {
            self.generate_particles();
        } else {
            self.update_particles();
        }

        true
    }

    fn render(&mut self, engine: &mut EngineState) -> bool {
        let secs = engine.timer.elapsed_seconds() as glsl::float;

        self.renders += 1;

        //let _dbg = self.debug(engine);
        //eprintln!("[RENDER] {_dbg}");

        if self.renders == 1 {
            engine.window.name = "Fireflies".to_string();
        }

        engine.render.clear();
        //engine.render.backcolor = rgba!(0.1, 0.2, 0.4, 0.0);
        engine.render.backcolor = rgba!(0.0, 0.0, 0.0, 1.0);

        for particle in self.particles.iter() {
            let pos = particle.position(secs);

            let mut color = particle.color;
            let alpha_speed = (particle.speed_c + particle.angle_s) / 2.0;
            let alpha_offset = particle.center_x;
            color.3 = secs
                .mul_add(alpha_speed, alpha_offset)
                .sin()
                .mul_add(0.5, 0.5);

            let size_speed = alpha_speed;
            let size_offset = particle.center_y;
            let size = secs
                .mul_add(size_speed, size_offset)
                .cos()
                .mul_add(particle.size / 2.0, particle.size)
                .round();

            engine.render.point(rge::Vertex {
                xyzw: vec4!(pos.0, pos.1, 0.0, size),
                rgba: color,
            });
        }

        true
    }
}

// ================================================================================================================================ //
