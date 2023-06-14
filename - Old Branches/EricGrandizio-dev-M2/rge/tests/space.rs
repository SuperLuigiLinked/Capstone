/*
 *  Crate: RGE
 *   Test: Space
 */

//! In this test, a window will be opened for the user to see.
//!
//! You should see an image of the Earth in Outer Space.
//! The Earth should emit a faint glow.
//! There should be asteroids orbiting around the Earth.
//! There should be a starry background that tracks to the Primary Monitor (or blackness if not on the Primary Monitor).
//!
//! Keyboard Controls:
//! * ESCAPE     : Toggle Fullscreen.
//! * LEFT/RIGHT : Rotate asteroid-belt.
//! * UP/DOWN    : Rotate asteroid-belt.
//!
//! Mouse Controls:
//! * SCROLL : Rotates the Earth.
//! * DRAG   : Causes a rainbow-trail to follow the cursor.
//! * LEFT   : Click or Hold to create particle effects at the cursor's location.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

// ================================================================================================================================ //

#![allow(clippy::identity_op)]

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 2, Story 5)"]
pub fn space() {
    test_main();
}

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

use rge::inputs::*;
#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4, Texture, Vertex, VertexUV, RGBA};
use rge::{EngineState, Game, GameEngine, GameEngineSettings};

use std::ops::Range;

use rand::prelude::*;

// ================================================================================================================================ //

fn test_main() {
    let mut app = App::new();

    let settings = GameEngineSettings {
        fps: 60.0,
        vsync: false,
        fullscreen: false,
        width: 640.0 * 1.5,
        height: 480.0 * 1.5,
    };

    let engine = GameEngine::new(&mut app, settings);
    engine.run()
}

// ================================================================================================================================ //

#[derive(Clone, Debug)]
struct Sprite {
    layer: glsl::float,
    center: glsl::vec2,
    size: glsl::vec2,
    rotation: glsl::float,
    color: glsl::vec4,
    uv: UV_PC,
    aspect_x: glsl::float,
    aspect_y: glsl::float,
    debug: bool,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            layer: 0.0,
            center: glsl::vec2(0.0, 0.0),
            size: glsl::vec2(1.0, 1.0),
            rotation: 0.0,
            color: glsl::vec4(1.0, 1.0, 1.0, 1.0),
            uv: UV_PC::default(),
            aspect_x: 1.0,
            aspect_y: 1.0,
            debug: false,
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Default)]
struct Particle {
    start_speed: glsl::float,
    start_color: glsl::vec4,

    center: glsl::vec2,
    speed: glsl::float,
    color: glsl::vec4,

    angle: glsl::float,
    size: usize,

    lifetime: usize,
    updates: usize,
}

impl Particle {
    pub fn random(rng: &mut ThreadRng, center: glsl::vec2) -> Self {
        let size = rng.gen_range(1..=4);
        let angle = rng.gen_range(0.0..glsl::float_consts::TAU);

        let start_speed = rng.gen_range((1.0 / 256.0)..=(4.0 / 256.0));
        let start_color = rgba!(1.0);

        let speed = start_speed;
        let color = start_color;

        let lifetime = rng.gen_range(10..=30);
        let updates = 0;

        Self {
            start_speed,
            start_color,

            center,
            speed,
            color,

            angle,
            size,

            lifetime,
            updates,
        }
    }

    pub fn update(&mut self, aspect_x: glsl::float, aspect_y: glsl::float) {
        self.updates += 1;

        let pc = (self.updates as glsl::float) / (self.lifetime as glsl::float);

        self.speed = self.start_speed * (1.0 - pc);
        let (sin, cos) = self.angle.sin_cos();

        self.center.0 += self.speed * cos * aspect_x;
        self.center.1 += self.speed * sin * aspect_y;

        let clear = rgba!(0.0, 0.0, 0.0, 0.0);
        self.color = utils::math::colerp(self.start_color, clear, pc);
    }

    pub fn alive(&self) -> bool {
        self.updates <= self.lifetime
    }
}

// ================================================================================================================================ //

struct App {
    updates: usize,
    renders: usize,

    texture: Texture,

    earth_rotation: glsl::float,

    yaw: glsl::float,
    pitch: glsl::float,
    roll: glsl::float,

    aspect_x: glsl::float,
    aspect_y: glsl::float,

    kc_left: bool,
    kc_right: bool,
    kc_up: bool,
    kc_down: bool,

    mb_left: bool,

    cursor: Point,
    cursor_history: Vec<(Point, usize)>,

    particles: Vec<Particle>,

    fullscreen: bool,
}

impl App {
    pub fn new() -> Self {
        let res = Texture::load_bytes(TEXTURE_FILE, None);
        let texture = res.unwrap();

        assert_eq!(texture.width(), TEXTURE_SIZE.0);
        assert_eq!(texture.height(), TEXTURE_SIZE.1);

        Self {
            updates: 0,
            renders: 0,
            texture,

            earth_rotation: 0.0,
            yaw: 0.0,
            pitch: -3.0 / 32.0,
            roll: 1.0 / 32.0,

            aspect_x: 1.0,
            aspect_y: 1.0,

            kc_up: false,
            kc_down: false,
            kc_left: false,
            kc_right: false,

            mb_left: false,

            fullscreen: false,

            cursor: Point::new(0.0, 0.0),
            cursor_history: Vec::with_capacity(64),

            particles: Vec::with_capacity(256),
        }
    }

    fn game_coords(engine: &EngineState, point: Point) -> glsl::vec2 {
        let window_size = engine.window.rect.size;

        let virtual_x = (point.x / window_size.w).mul_add(2.0, -1.0) as glsl::float;
        let virtual_y = (point.y / window_size.h).mul_add(2.0, -1.0) as glsl::float;
        glsl::vec2(virtual_x, virtual_y)
    }
}

// ================================================================================================================================ //

const TEXTURE_FILE: &[u8] = include_bytes!("../images/space.png");

const TEXTURE_SIZE: (usize, usize) = (2560, 2560);

#[allow(non_camel_case_types)]
type UV_PX = (Range<usize>, Range<usize>);
#[allow(non_camel_case_types)]
type UV_PC = (Range<glsl::float>, Range<glsl::float>);

fn texture_uv(uv_px: UV_PX) -> UV_PC {
    let tw = TEXTURE_SIZE.0 as glsl::float;
    let th = TEXTURE_SIZE.1 as glsl::float;

    let xs = uv_px.0.start as glsl::float;
    let xe = uv_px.0.end as glsl::float;
    let ys = uv_px.1.start as glsl::float;
    let ye = uv_px.1.end as glsl::float;

    let u_range = (xs / tw)..(xe / tw);
    let v_range = (ys / th)..(ye / th);
    (u_range, v_range)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

const EARTH_PX: UV_PX = (0..1280, 0..1280);

const ASTEROID_PX: UV_PX = (1280..2560, 0..1280);

const STARS_PX: UV_PX = (0..1280, 1280..(1280 + 935));

// ================================================================================================================================ //

impl Game for App {
    fn update(&mut self, engine: &mut EngineState) -> bool {
        self.updates += 1;

        let _dbg = self.debug(engine);
        //eprintln!("[UPDATE] {_dbg}");

        let mut rng = thread_rng();

        // ---------------------------------------------------------------- //

        self.cursor_history
            .retain(|(_, update)| self.updates - update < 5);

        for input in engine.inputs.events.iter() {
            match input {
                Input::ScrollWheel {
                    delta_x: _,
                    delta_y,
                } => {
                    self.earth_rotation += (*delta_y as glsl::float) / 45.0;
                }
                Input::KeyPress {
                    keycode: KC_UP,
                    pressed,
                } => {
                    self.kc_up = *pressed;
                }
                Input::KeyPress {
                    keycode: KC_DOWN,
                    pressed,
                } => {
                    self.kc_down = *pressed;
                }
                Input::KeyPress {
                    keycode: KC_LEFT,
                    pressed,
                } => {
                    self.kc_left = *pressed;
                }
                Input::KeyPress {
                    keycode: KC_RIGHT,
                    pressed,
                } => {
                    self.kc_right = *pressed;
                }
                Input::KeyPress {
                    keycode: KC_ESCAPE,
                    pressed: true,
                } => {
                    self.fullscreen = !self.fullscreen;
                }
                Input::ButtonPress {
                    button: MB_LEFT,
                    pressed,
                } => {
                    self.mb_left = *pressed;
                }
                Input::CursorMove { point } => {
                    self.cursor_history.push((*point, self.updates));
                    self.cursor = *point;
                }
                _ => {}
            }
        }

        // ---------------------------------------------------------------- //

        let rotate_speed = 1.0 / 120.0;

        if self.kc_up {
            self.roll -= rotate_speed
        };
        if self.kc_down {
            self.roll += rotate_speed;
        }
        if self.kc_left {
            self.pitch -= rotate_speed;
        }
        if self.kc_right {
            self.pitch += rotate_speed;
        }

        // ---------------------------------------------------------------- //

        if self.mb_left {
            let virtual_cursor = Self::game_coords(engine, self.cursor);

            for _ in 0..8 {
                let particle = Particle::random(&mut rng, virtual_cursor);
                self.particles.push(particle);
            }
        }

        for particle in self.particles.iter_mut() {
            particle.update(self.aspect_x, self.aspect_y);
        }
        self.particles.retain(|particle| particle.alive());

        // ---------------------------------------------------------------- //

        true
    }

    fn render(&mut self, engine: &mut EngineState) -> bool {
        self.renders += 1;

        let _dbg = self.debug(engine);
        //eprintln!("[RENDER] {_dbg}");
        engine.window.name = _dbg;

        let secs = engine.timer.elapsed_seconds() as glsl::float;

        //let aspect = engine.window.rect.aspect() as glsl::float;
        let rect = engine.window.rect;
        let aspect_y = (rect.size.w / rect.size.h).min(1.0) as glsl::float;
        let aspect_x = (rect.size.h / rect.size.w).min(1.0) as glsl::float;
        //eprintln!("ASPECT: | {:8.5} | {:8.5} |", aspect_y, aspect_x);
        self.aspect_x = aspect_x;
        self.aspect_y = aspect_y;

        // ---------------------------------------------------------------- //

        engine.window.fullscreen = self.fullscreen;

        // ---------------------------------------------------------------- //

        let earth_uv = self::texture_uv(EARTH_PX);
        let asteroid_uv = self::texture_uv(ASTEROID_PX);
        let stars_uv = self::texture_uv(STARS_PX);

        // ---------------------------------------------------------------- //

        engine.render.clear();

        if self.renders == 1 {
            engine.update_atlas(&self.texture);
        }

        self.yaw = secs / -2.0;

        let earth_radius = 1.25;

        // ---------------------------------------------------------------- //

        const ASTEROID_COUNT: usize = 8;
        let mut asteroids = vec![Sprite::default(); ASTEROID_COUNT];

        for (idx, asteroid) in asteroids.iter_mut().enumerate() {
            let pc = (idx as glsl::float) / (ASTEROID_COUNT as glsl::float);

            let rot_y = self.yaw + pc;
            let rot_p = self.pitch;
            let rot_r = self.roll;

            let glsl::vec4(asteroid_x, asteroid_y, asteroid_z, asteroid_s) =
                self::ring_pos(rot_y, rot_p, rot_r, vec3!(), 0.75, 1.0, -0.5);

            let sprite = Sprite {
                layer: asteroid_z * earth_radius,
                center: vec2!(asteroid_x * earth_radius, asteroid_y * earth_radius),
                size: vec2!(
                    0.2 * asteroid_s * earth_radius,
                    0.2 * asteroid_s * earth_radius
                ),
                rotation: (secs / 4.0) + pc,
                color: rgba!(1.0),
                uv: asteroid_uv.clone(),
                aspect_x,
                aspect_y,
                debug: false,
            };

            *asteroid = sprite;
        }

        // ---------------------------------------------------------------- //

        let earth = Sprite {
            layer: 0.0,
            center: vec2!(0.0, 0.0),
            size: vec2!(earth_radius, earth_radius),
            rotation: self.earth_rotation,
            color: rgba!(1.0),
            uv: earth_uv,
            aspect_x,
            aspect_y,
            debug: false,
        };

        const EARTH_FAN_SIZE: usize = 640;
        let mut earth_light: [Vertex; EARTH_FAN_SIZE + 2] = unsafe { core::mem::zeroed() };
        earth_light[0] = Vertex {
            xyzw: vec4!(0.0, 0.0),
            rgba: rgba!(1.0, 0.25),
        };
        let light_scale = 0.55;

        for i in 0..=EARTH_FAN_SIZE {
            let pc = (i as glsl::float) / (EARTH_FAN_SIZE as glsl::float);
            let pos = utils::math::circle_point(
                earth.center,
                earth.size.0 * light_scale * aspect_x,
                earth.size.1 * light_scale * aspect_y,
                pc,
            );

            earth_light[i + 1] = Vertex {
                xyzw: vec4!(pos.0, pos.1),
                rgba: rgba!(1.0, 0.0),
            };
        }

        // ---------------------------------------------------------------- //

        let primary_screen = &engine.inputs.screens[0];
        let screen_rect = primary_screen.rect;
        let screen_size = screen_rect.size;
        let screen_center = screen_rect.center();

        let window_rect = &engine.window.rect;
        let window_size = window_rect.size;
        let window_center = window_rect.center();

        let sx = screen_center.x;
        let sy = screen_center.y;
        let sw = screen_size.w / 2.0;
        let sh = screen_size.h / 2.0;

        let wx = window_center.x;
        let wy = window_center.y;
        let ww = window_size.w / 2.0;
        let wh = window_size.h / 2.0;

        let offs_l = (-1.0 + (((sx - sw) - (wx - ww)) / ww)) as glsl::float;
        let offs_r = (1.0 + (((sx + sw) - (wx + ww)) / ww)) as glsl::float;
        let offs_t = (-1.0 + (((sy - sh) - (wy - wh)) / wh)) as glsl::float;
        let offs_b = (1.0 + (((sy + sh) - (wy + wh)) / wh)) as glsl::float;

        let stars_color = 0.25;

        let stars = [
            VertexUV {
                xyzw: vec4!(offs_l, offs_t),
                rgba: rgba!(stars_color),
                uv: vec2!(stars_uv.0.start, stars_uv.1.start),
            },
            VertexUV {
                xyzw: vec4!(offs_r, offs_t),
                rgba: rgba!(stars_color),
                uv: vec2!(stars_uv.0.end, stars_uv.1.start),
            },
            VertexUV {
                xyzw: vec4!(offs_r, offs_b),
                rgba: rgba!(stars_color),
                uv: vec2!(stars_uv.0.end, stars_uv.1.end),
            },
            VertexUV {
                xyzw: vec4!(offs_l, offs_b),
                rgba: rgba!(stars_color),
                uv: vec2!(stars_uv.0.start, stars_uv.1.end),
            },
        ];

        // ---------------------------------------------------------------- //

        let mut sprites = Vec::with_capacity(128);

        sprites.push(earth);
        sprites.append(&mut asteroids);

        // ---------------------------------------------------------------- //

        engine.render.uv_triangle_fan(&stars);
        engine.render.triangle_fan(&earth_light);

        // Sort Sprites from Back-to-Front.
        sprites.sort_by(|a, b| (b.layer).total_cmp(&a.layer));

        for sprite in sprites {
            Self::draw_sprite(engine, &sprite);
        }

        // ---------------------------------------------------------------- //

        let line_iter = self.cursor_history.windows(2);
        let line_count = line_iter.clone().count();

        for (idx, line) in line_iter.rev().enumerate() {
            let pc_a = (idx as glsl::float) / (line_count as glsl::float);
            let pc_b = ((idx + 1) as glsl::float) / (line_count as glsl::float);

            let vec_a = Self::game_coords(engine, line[0].0);
            let vec_b = Self::game_coords(engine, line[1].0);

            let vertex_a = Vertex {
                xyzw: vec4!(vec_a.0, vec_a.1),
                rgba: utils::math::hue(pc_a),
            };

            let vertex_b = Vertex {
                xyzw: vec4!(vec_b.0, vec_b.1),
                rgba: utils::math::hue(pc_b),
            };

            engine.render.line(&[vertex_a, vertex_b]);
        }

        // ---------------------------------------------------------------- //

        for particle in self.particles.iter() {
            engine.render.point(Vertex {
                xyzw: vec4!(
                    particle.center.0,
                    particle.center.1,
                    0.0,
                    particle.size as glsl::float
                ),
                rgba: particle.color,
            })
        }

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

    fn draw_sprite(engine: &mut EngineState, sprite: &Sprite) {
        let radius = (2.0 as glsl::float).sqrt();
        let half_w = (sprite.size.0 * 0.5) * radius;
        let half_h = (sprite.size.1 * 0.5) * radius;

        let rotation_tl = 5.0 / 8.0;
        let rotation_tr = 7.0 / 8.0;
        let rotation_bl = 3.0 / 8.0;
        let rotation_br = 1.0 / 8.0;

        let offset_tl = utils::math::circle_point(sprite.center, half_w, half_h, rotation_tl);
        let offset_tr = utils::math::circle_point(sprite.center, half_w, half_h, rotation_tr);
        let offset_bl = utils::math::circle_point(sprite.center, half_w, half_h, rotation_bl);
        let offset_br = utils::math::circle_point(sprite.center, half_w, half_h, rotation_br);

        let mut pos_tl = utils::math::rotate_about(offset_tl, sprite.center, sprite.rotation);
        let mut pos_tr = utils::math::rotate_about(offset_tr, sprite.center, sprite.rotation);
        let mut pos_bl = utils::math::rotate_about(offset_bl, sprite.center, sprite.rotation);
        let mut pos_br = utils::math::rotate_about(offset_br, sprite.center, sprite.rotation);

        // Correct Aspect-Ratio.
        {
            pos_tl.0 *= sprite.aspect_x;
            pos_tr.0 *= sprite.aspect_x;
            pos_bl.0 *= sprite.aspect_x;
            pos_br.0 *= sprite.aspect_x;

            pos_tl.1 *= sprite.aspect_y;
            pos_tr.1 *= sprite.aspect_y;
            pos_bl.1 *= sprite.aspect_y;
            pos_br.1 *= sprite.aspect_y;
        }

        let uv_tl = glsl::vec2(sprite.uv.0.start, sprite.uv.1.start);
        let uv_tr = glsl::vec2(sprite.uv.0.end, sprite.uv.1.start);
        let uv_bl = glsl::vec2(sprite.uv.0.start, sprite.uv.1.end);
        let uv_br = glsl::vec2(sprite.uv.0.end, sprite.uv.1.end);

        let tl = VertexUV {
            xyzw: vec4!(pos_tl.0, pos_tl.1),
            rgba: sprite.color,
            uv: uv_tl,
        };
        let tr = VertexUV {
            xyzw: vec4!(pos_tr.0, pos_tr.1),
            rgba: sprite.color,
            uv: uv_tr,
        };
        let bl = VertexUV {
            xyzw: vec4!(pos_bl.0, pos_bl.1),
            rgba: sprite.color,
            uv: uv_bl,
        };
        let br = VertexUV {
            xyzw: vec4!(pos_br.0, pos_br.1),
            rgba: sprite.color,
            uv: uv_br,
        };

        if sprite.layer > 0.0 {
            let vertices = [tl, tr, br, bl];
            engine.render.uv_triangle_fan(&vertices);
        } else {
            let vertices = [tl, tr, bl, br];
            engine.render.uv_triangle_strip(&vertices)
        }

        if sprite.debug {
            let point = |vertex: VertexUV, color: glsl::vec4| -> Vertex {
                Vertex {
                    xyzw: vec4!(vertex.xyzw.0, vertex.xyzw.1, vertex.xyzw.2, 4.0),
                    rgba: color,
                }
            };
            engine.render.point(point(tl, rgba!(1.0, 0.0, 0.0)));
            engine.render.point(point(tr, rgba!(0.0, 1.0, 0.0)));
            engine.render.point(point(bl, rgba!(0.0, 0.0, 1.0)));
            engine.render.point(point(br, rgba!(1.0, 1.0, 0.0)));
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //
}

// ================================================================================================================================ //

#[allow(unused)]
fn round(val: glsl::float) -> glsl::float {
    (val + 0.5).floor()
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
fn mod_r(a: glsl::float, b: glsl::float) -> glsl::float {
    a - b * self::round(a / b)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
fn ring_pos(
    rot_y: glsl::float,
    rot_p: glsl::float,
    rot_r: glsl::float,
    pos_origin: glsl::vec3,
    pos_radius: glsl::float,
    scl_origin: glsl::float,
    scl_radius: glsl::float,
) -> glsl::vec4 {
    let glsl::vec3(x, y, z) = utils::math::xyz_rotation(rot_y, rot_p, rot_r);
    let nx = x.mul_add(pos_radius, pos_origin.0);
    let ny = y.mul_add(pos_radius, pos_origin.1);
    let nz = z.mul_add(pos_radius, pos_origin.2);

    let scale = z.mul_add(scl_radius, scl_origin);

    glsl::vec4(nx, ny, nz, scale)
}

// ================================================================================================================================ //
