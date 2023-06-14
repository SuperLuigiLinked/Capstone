/*
 *  Crate: RGE
 *   Test: Game-of-Life
 */

//! In this test, a window will be opened for the user to see.
//!
//! Conway's "Game of Life" will play out within the window.
//!
//! The texture for the board is rendered almost entirely on the CPU.
//! It is then uploaded to the GPU to be mapped to some triangles and stretched to fit to the window.
//! As such, this test exemplifies how this library can also be used for Software-Rendering.
//!
//! Keyboard Controls:
//! * SPACEBAR   : Randomize the Board State.
//! * DELETE     : Clear the Board State.
//! * SHIFT      : Toggle Rainbow Mode.
//! * ESCAPE     : Toggle Window Fullscreen.
//! * BACKSPACE  : Toggle Vsync.
//! * UP/DOWN    : Increase/Decrease the Simulation Speed (aka, the FPS).
//! * ENTER      : Pause/Unpause the Simulation.
//! * PERIOD     : Advance the Simulation by 1 Update (if Paused).

//! Mouse Controls:
//! * LEFT/RIGHT : Toggle Cells On/Off.
//!
//! When the user is done observing the program, they may close the window.
//!
//! If no failures occur, and the program runs and shuts down as expected, then the Test has PASSED.

// ================================================================================================================================ //

mod utils;

// ================================================================================================================================ //

#[test]
#[ignore = "User Acceptance Test (Milestone 2, Story 4)"]
pub fn game_of_life() {
    test_main();
}

// -------------------------------------------------------------------------------------------------------------------------------- //

// Change the below parameters as you wish.

// Make sure to run in `--release` mode if you wish for a larger or faster game.
// Be careful that WINDOW_SIZE does not exceed the size of your monitor.

/// Frames-per-Second of the simulation. [A value of `0.0` means to match the Monitor Refresh-Rate]
const FPS: f64 = 30.0;

/// Whether or not to wait for VSYNC before advancing. [Set to `false` if you wish for an FPS higher than your Monitor's Refresh-Rate]
const VSYNC: bool = true;

/// Whether or not to open in Fullscreen mode.
const FULLSCREEN: bool = false;

/// Size of the Game Board, in Cells.
const GAME_SIZE: (usize, usize) = (256, 256);

/// Size of a Cell, in Pixels. [May be changed at runtime by resizing the Window]
const SCALE_SIZE: (f64, f64) = (3.0, 3.0);

/// Size of the Window that is opened. [May be changed at runtime by resizing the Window]
const WINDOW_SIZE: (f64, f64) = (
    GAME_SIZE.0 as f64 * SCALE_SIZE.0,
    GAME_SIZE.1 as f64 * SCALE_SIZE.1,
);

/// Whether or not Cell-Neighbors wrap-around the edges of the Board.
const WRAPAROUND: bool = true;

/// Whether or not to Shade the Board in Rainbow Colors and do perspective-warping effects.
const RAINBOW: bool = false;

/// Whether or not to print Benchmarking results to the Console.
const BENCHMARK: bool = false;

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

use rge::inputs::*;
#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4, Vertex, VertexUV};
use rge::{EngineState, Game, GameEngine, GameEngineSettings, Texture, RGBA};

use rand::Rng;

use std::ops::Range;
use std::time::{Duration, Instant};

// ================================================================================================================================ //

fn test_main() {
    let mut app = App::new();

    let settings = GameEngineSettings {
        fps: FPS,
        vsync: VSYNC,
        fullscreen: FULLSCREEN,
        width: WINDOW_SIZE.0,
        height: WINDOW_SIZE.1,
    };

    let engine = GameEngine::new(&mut app, settings);
    engine.run()
}

// ================================================================================================================================ //

struct App {
    mb_right: bool,
    mb_left: bool,
    cursor: Point,

    fullscreen: bool,
    rainbow: bool,

    paused: bool,
    game: GameOfLife,

    renders: usize,
    updates: usize,
}

impl App {
    pub fn new() -> Self {
        let updates = 0;
        let renders = 0;

        let game = GameOfLife::new(GAME_SIZE.0, GAME_SIZE.1, WRAPAROUND);
        let paused = false;

        let rainbow = RAINBOW;
        let fullscreen = FULLSCREEN;

        let cursor = Point::default();
        let mb_right = false;
        let mb_left = false;

        Self {
            mb_right,
            mb_left,
            cursor,

            fullscreen,
            rainbow,

            paused,
            game,
            renders,
            updates,
        }
    }

    fn game_coords(
        &self,
        engine: &EngineState,
        window_coords: Point,
    ) -> Option<(usize, usize, usize)> {
        let Size {
            w: window_w,
            h: window_h,
        } = engine.window.rect.size;

        let Point {
            x: cursor_x,
            y: cursor_y,
        } = window_coords;

        let (game_w, game_h) = (self.game.width, self.game.height);
        let (scale_x, scale_y) = (window_w / (game_w as f64), window_h / (game_h as f64));

        let (virtual_x, virtual_y) = (cursor_x / scale_x, cursor_y / scale_y);

        let (index_x, index_y) = (virtual_x as isize, virtual_y as isize);

        let inbounds = (index_x >= 0)
            && ((index_x as usize) < game_w)
            && (index_y >= 0)
            && ((index_y as usize) < game_h);

        if inbounds {
            let x = index_x as usize;
            let y = index_y as usize;
            let i = y * game_w + x;
            Some((x, y, i))
        } else {
            None
        }
    }
}

// ================================================================================================================================ //

impl App {
    pub fn debug(&self, engine: &EngineState) -> String {
        let secs = engine.timer.elapsed_seconds();
        let expected = engine.timer.elapsed_frames() + 1.0;
        let dropped = (expected as isize) - (self.updates as isize);

        let fps = engine.timer.fps();
        let vsync = engine.timer.vsync();
        let renders = self.renders;
        let updates = self.updates;

        let fps_string = if self.paused {
            "<Paused>".to_string()
        } else if fps == 0.0 {
            "<Uncapped>".to_string()
        } else {
            fps.to_string()
        };

        format!("RGE | FPS: {fps_string} | VSYNC: {vsync} | Renders: {renders} | Updates: {updates} | Expected: {expected:.2} | Dropped: {dropped} | Seconds: {secs:.2}")
    }

    fn benchmark(callback: impl FnOnce()) -> Duration {
        let t1 = Instant::now();

        callback();

        let t2 = Instant::now();
        t2.saturating_duration_since(t1)
    }

    fn print_benchmark(label: &str, benchmark: Duration) {
        if BENCHMARK {
            let elapsed = benchmark.as_nanos();
            eprintln!("{label:<14} {elapsed:>9} NS");
        }
    }
}

// ================================================================================================================================ //

impl Game for App {
    fn update(&mut self, engine: &mut EngineState) -> bool {
        self.updates += 1;

        let _dbg = self.debug(engine);
        //eprintln!("[UPDATE] {_dbg}");

        // -------------------------------- //

        let spacebar_pressed = engine.inputs.contains(Input::KeyPress {
            keycode: KC_SPACE,
            pressed: true,
        });

        let up_presses = engine.inputs.count(Input::KeyPress {
            keycode: KC_UP,
            pressed: true,
        });

        let down_presses = engine.inputs.count(Input::KeyPress {
            keycode: KC_DOWN,
            pressed: true,
        });

        let backspace_pressed = engine.inputs.contains(Input::KeyPress {
            keycode: KC_BACKSPACE,
            pressed: true,
        });

        let shift_pressed = engine.inputs.contains(Input::KeyPress {
            keycode: KC_SHIFT,
            pressed: true,
        }) || engine.inputs.contains(Input::KeyPress {
            keycode: KC_LSHIFT,
            pressed: true,
        }) || engine.inputs.contains(Input::KeyPress {
            keycode: KC_RSHIFT,
            pressed: true,
        });

        let delete_pressed = engine.inputs.contains(Input::KeyPress {
            keycode: KC_DELETE,
            pressed: true,
        });

        let escape_pressed = engine.inputs.contains(Input::KeyPress {
            keycode: KC_ESCAPE,
            pressed: true,
        });

        let enter_pressed = engine.inputs.contains(Input::KeyPress {
            keycode: KC_ENTER,
            pressed: true,
        });

        let period_pressed = engine.inputs.contains(Input::KeyPress {
            keycode: KC_PERIOD,
            pressed: true,
        });

        for input in engine.inputs.events.iter() {
            match input {
                Input::ButtonPress {
                    button: MB_LEFT,
                    pressed,
                } => {
                    self.mb_left = *pressed;
                }
                Input::ButtonPress {
                    button: MB_RIGHT,
                    pressed,
                } => {
                    self.mb_right = *pressed;
                }
                _ => {}
            }
            if let Input::CursorMove { point } = input {
                self.cursor = *point;
            }
        }

        let cursor = self.game_coords(engine, self.cursor);

        // -------------------------------- //

        if shift_pressed {
            self.rainbow = !self.rainbow;
        }
        if escape_pressed {
            self.fullscreen = !self.fullscreen;
        }
        if backspace_pressed {
            engine.timer.toggle_vsync(!engine.timer.vsync());
        }
        if enter_pressed {
            self.paused = !self.paused;
        }

        let fps_adjust = (up_presses as isize).wrapping_sub(down_presses as isize);
        if fps_adjust != 0 {
            let old_fps = engine.timer.fps();
            let new_fps = (old_fps + fps_adjust as f64).max(0.0);

            engine.timer.reset_fps(new_fps);
        }

        // -------------------------------- //

        let update_time = Self::benchmark(|| {
            if spacebar_pressed || (self.updates == 1) {
                self.game.randomize();
            } else if delete_pressed {
                self.game.clear();
            } else if !self.paused || period_pressed {
                self.game.update();
            }

            if let Some((x, y, _index)) = cursor {
                if self.mb_left {
                    self.game.overwrite(x, y, true);
                }
                if self.mb_right {
                    self.game.overwrite(x, y, false);
                }
            }
        });
        Self::print_benchmark("[LIFE UPDATE]", update_time);

        let render_time = Self::benchmark(|| {
            self.game.render();

            if self.mb_left || self.mb_right {
                if let Some((x, y, index)) = cursor {
                    let alive = self.game.read(x, y);

                    let pixels = self.game.image.as_mut_slice();
                    assert!(index < pixels.len());

                    if alive {
                        pixels[index] = rgba!(1.0, 0.5, 0.5, 1.0).into();
                    } else {
                        pixels[index] = rgba!(1.0, 0.0, 0.0, 1.0).into();
                    }
                }
            }
        });
        Self::print_benchmark("[LIFE RENDER]", render_time);

        let upload_time = Self::benchmark(|| {
            engine.update_atlas(&self.game.image);
        });
        Self::print_benchmark("[GPU UPLOAD]", upload_time);

        // -------------------------------- //

        true
    }

    fn render(&mut self, engine: &mut EngineState) -> bool {
        self.renders += 1;

        let _dbg = self.debug(engine);
        //eprintln!("[RENDER] {_dbg}");
        engine.window.name = _dbg;

        // -------------------------------- //

        engine.render.clear();

        if self.rainbow {
            self.shade_rainbow(engine);
        } else {
            self.shade_normal(engine);
        }

        engine.window.fullscreen = self.fullscreen;

        // -------------------------------- //

        true
    }
}

impl App {
    fn shade_normal(&mut self, engine: &mut EngineState) {
        engine.render.uv_triangle_strip(&[
            VertexUV {
                xyzw: vec4!(-1.0, -1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(0.0, 0.0),
            },
            VertexUV {
                xyzw: vec4!(1.0, -1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(1.0, 0.0),
            },
            VertexUV {
                xyzw: vec4!(-1.0, 1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(0.0, 1.0),
            },
            VertexUV {
                xyzw: vec4!(1.0, 1.0),
                rgba: rgba!(1.0, 1.0, 1.0, 1.0),
                uv: vec2!(1.0, 1.0),
            },
        ]);
    }

    fn shade_rainbow(&mut self, engine: &mut EngineState) {
        // ---------------------------------------------------------------- //

        #[allow(unused)]
        let map_box = |pc: glsl::float, radius: glsl::float, origin: glsl::float| {
            let radians = pc * glsl::float_consts::TAU;
            let (sin, cos) = radians.sin_cos();

            let x_dist = (radius / cos).abs();
            let y_dist = (radius / sin).abs();
            let dist = x_dist.min(y_dist);

            let x = cos.mul_add(dist, origin);
            let y = sin.mul_add(dist, origin);

            (x, y)
        };

        #[allow(unused)]
        let map_circle = |pc: glsl::float, radius: glsl::float, origin: glsl::float| {
            let radians = pc * glsl::float_consts::TAU;
            let (sin, cos) = radians.sin_cos();

            let x = cos.mul_add(radius, origin);
            let y = sin.mul_add(radius, origin);

            (x, y)
        };

        // ---------------------------------------------------------------- //

        let xy_pc = |pc: glsl::float| {
            let (x, y) = map_box(pc, 1.0, 0.0);
            vec4!(x, y)
        };

        let rgba_pc = utils::math::hue;

        let uv_pc = |pc: glsl::float| {
            let (x, y) = map_circle(pc, 0.5, 0.5);
            vec2!(x, y)
        };

        // ---------------------------------------------------------------- //

        let rot_offs = (engine.timer.next_seconds() / 8.0) as glsl::float;

        const POINT_COUNT: usize = 640;
        let mut points: [VertexUV; POINT_COUNT + 2] = unsafe { core::mem::zeroed() };

        points[0] = VertexUV {
            xyzw: vec4!(0.0, 0.0),
            rgba: rgba!(0.5),
            uv: vec2!(0.5, 0.5),
        };
        for i in 0..=POINT_COUNT {
            let pc = (i as glsl::float) / (POINT_COUNT as glsl::float);

            points[i + 1] = VertexUV {
                xyzw: xy_pc(pc),
                rgba: rgba_pc(pc + rot_offs),
                uv: uv_pc(pc - rot_offs / 8.0),
            };
        }

        engine.render.uv_triangle_fan(&points);

        // ---------------------------------------------------------------- //
    }
}

// ================================================================================================================================ //

struct GameOfLife {
    /// Width of the Board.
    width: usize,

    /// Height of the Board.
    height: usize,

    /// Whether or not Cells wrap around the Board.
    wraparound: bool,

    /// Readable Cells.
    r_cells: Vec<bool>,

    /// Writable Cells.
    w_cells: Vec<bool>,

    /// Colored Image.
    image: Texture,
}

impl GameOfLife {
    /// Whether or not redundant bounds-checking is done on array accesses to ensure correctness. [`false` is faster]
    const CHECKED: bool = cfg!(debug_assertions);

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Constructs a new Game of Life simulator.
    /// # Panics
    /// Panics if `width` or `height` are zero.
    pub fn new(width: usize, height: usize, wraparound: bool) -> Self {
        assert_ne!(width, 0, "Width must be Non-Zero");
        assert_ne!(height, 0, "Height must be Non-Zero");

        let length_res = width.checked_mul(height);
        let length = length_res.expect("Board too big");

        let r_cells = vec![false; length];
        let w_cells = vec![false; length];
        let image = Texture::new(width, height);

        Self {
            width,
            height,
            wraparound,
            r_cells,
            w_cells,
            image,
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Returns `a mod b`, where the `result >= 0`, as if both `a` and `b` were signed values.
    /// # SAFETY
    /// `b` must be Non-Zero.
    #[inline]
    const unsafe fn modulo(a: usize, b: usize) -> usize {
        if b == 0 {
            std::hint::unreachable_unchecked()
        } else {
            (a as isize).wrapping_rem_euclid(b as isize) as usize
        }
    }

    /// Returns the Next State of a cell, given its previous state.
    #[inline]
    const fn next_state(alive: bool, neighbors: u8) -> bool {
        (neighbors == 3) || (alive && (neighbors == 2))
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Returns whether or not the given 2D-Coordinates are In-Bounds.
    #[inline]
    const fn is_inbounds(&self, x: usize, y: usize) -> bool {
        (x < self.width) && (y < self.height)
    }

    /// Calculates the Index of the 2D-Coordinates.
    /// # Safety
    /// `x` and `y` must be in-bounds.
    #[inline]
    const unsafe fn index_of(&self, x: usize, y: usize) -> usize {
        y.wrapping_mul(self.width).wrapping_add(x)
    }

    /// Calculates the Wrapped-Index of the 2D-Coordinates.
    #[inline]
    const fn index_of_wrapping(&self, x: usize, y: usize) -> usize {
        // SAFETY: `width` and `height` are guaranteed to be Non-Zero.
        let wx = unsafe { Self::modulo(x, self.width) };
        let wy = unsafe { Self::modulo(y, self.height) };
        unsafe { self.index_of(wx, wy) }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Returns the Range of X-Coordinates.
    #[inline]
    const fn x_range(&self) -> Range<usize> {
        0..self.width
    }

    /// Returns the Range of Inner X-Coordinates.
    #[inline]
    const fn x_range_inner(&self) -> Range<usize> {
        1..(self.width - 1)
    }

    /// Returns the Range of Y-Coordinates.
    #[inline]
    const fn y_range(&self) -> Range<usize> {
        0..self.height
    }

    /// Returns the Range of Inner Y-Coordinates.
    #[inline]
    const fn y_range_inner(&self) -> Range<usize> {
        1..(self.height - 1)
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Reads the Cell at the 2D-Coordinate `(x, y)`.\
    /// Returns `false` if not in-bounds.
    pub fn read(&self, x: usize, y: usize) -> bool {
        if self.is_inbounds(x, y) {
            let index = unsafe { self.index_of(x, y) };
            self.r_cells[index]
        } else {
            false
        }
    }

    /// Writes to the Cell at the 2D-Coordinate `(x, y)`.\
    /// Does nothing if not in-bounds.
    fn write(&mut self, x: usize, y: usize, val: bool) {
        if self.is_inbounds(x, y) {
            let index = unsafe { self.index_of(x, y) };
            self.w_cells[index] = val;
        }
    }

    /// Reads the Cell at the 2D-Coordinate `(x, y)`.
    /// # Safety
    /// `x` and `y` must be in-bounds.
    unsafe fn read_unchecked(&self, x: usize, y: usize) -> bool {
        let index = self.index_of(x, y);
        *self.r_cells.get_unchecked(index)
    }

    /// Writes to the Cell at the 2D-Coordinate `(x, y)`.
    /// # Safety
    /// `x` and `y` must be in-bounds.
    unsafe fn write_unchecked(&mut self, x: usize, y: usize, val: bool) {
        let index = self.index_of(x, y);
        *self.w_cells.get_unchecked_mut(index) = val;
    }

    /// Reads the Cell at the 2D-Coordinate `(x, y)`.\
    /// Wraps around the edges of the Board if not in-bounds.
    fn read_wrapping(&self, x: usize, y: usize) -> bool {
        let index = self.index_of_wrapping(x, y);
        unsafe { *self.r_cells.get_unchecked(index) }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Returns the Neighbor of the Cell at the 2D-Coordinate, at the given offset.\
    /// Returns false if not in-bounds.
    fn neighbor_at(&self, x: usize, y: usize, ox: isize, oy: isize) -> bool {
        let nx = x.wrapping_add_signed(ox);
        let ny = y.wrapping_add_signed(oy);

        self.read(nx, ny)
    }

    /// Returns the Neighbor of the Cell at the 2D-Coordinate, at the given offset.
    /// # Safety
    /// `x` and `y` must be in-bounds.
    unsafe fn neighbor_at_unchecked(&self, x: usize, y: usize, ox: isize, oy: isize) -> bool {
        let nx = x.wrapping_add_signed(ox);
        let ny = y.wrapping_add_signed(oy);

        self.read_unchecked(nx, ny)
    }

    /// Returns the Neighbor of the Cell at the 2D-Coordinate, at the given offset.\
    /// Wraps around the edges of the Board if not in-bounds.
    fn neighbor_at_wrapping(&self, x: usize, y: usize, ox: isize, oy: isize) -> bool {
        let nx = x.wrapping_add_signed(ox);
        let ny = y.wrapping_add_signed(oy);

        self.read_wrapping(nx, ny)
    }

    /// Gets the Neighbor count of the Cell at the 2D-Coordinate.
    #[rustfmt::skip]
    fn neighbors(&self, x: usize, y: usize) -> u8 {
                     (self.neighbor_at(x, y, -1, -1) as u8)
        .wrapping_add(self.neighbor_at(x, y,  0, -1) as u8)
        .wrapping_add(self.neighbor_at(x, y,  1, -1) as u8)
        .wrapping_add(self.neighbor_at(x, y, -1,  0) as u8)
        .wrapping_add(self.neighbor_at(x, y,  1,  0) as u8)
        .wrapping_add(self.neighbor_at(x, y, -1,  1) as u8)
        .wrapping_add(self.neighbor_at(x, y,  0,  1) as u8)
        .wrapping_add(self.neighbor_at(x, y,  1,  1) as u8)
    }

    /// Gets the Neighbor count of the Cell at the 2D-Coordinate.
    /// # Safety
    /// `x` and `y` must be in-bounds.
    #[rustfmt::skip]
    unsafe fn neighbors_unchecked(&self, x: usize, y: usize) -> u8 {
                     (self.neighbor_at_unchecked(x, y, -1, -1) as u8)
        .wrapping_add(self.neighbor_at_unchecked(x, y,  0, -1) as u8)
        .wrapping_add(self.neighbor_at_unchecked(x, y,  1, -1) as u8)
        .wrapping_add(self.neighbor_at_unchecked(x, y, -1,  0) as u8)
        .wrapping_add(self.neighbor_at_unchecked(x, y,  1,  0) as u8)
        .wrapping_add(self.neighbor_at_unchecked(x, y, -1,  1) as u8)
        .wrapping_add(self.neighbor_at_unchecked(x, y,  0,  1) as u8)
        .wrapping_add(self.neighbor_at_unchecked(x, y,  1,  1) as u8)
    }

    /// Gets the Neighbor count of the Cell at the 2D-Coordinate, wrapping around the edges of the Board.
    #[rustfmt::skip]
    fn neighbors_wrapping(&self, x: usize, y: usize) -> u8 {
                     (self.neighbor_at_wrapping(x, y, -1, -1) as u8)
        .wrapping_add(self.neighbor_at_wrapping(x, y,  0, -1) as u8)
        .wrapping_add(self.neighbor_at_wrapping(x, y,  1, -1) as u8)
        .wrapping_add(self.neighbor_at_wrapping(x, y, -1,  0) as u8)
        .wrapping_add(self.neighbor_at_wrapping(x, y,  1,  0) as u8)
        .wrapping_add(self.neighbor_at_wrapping(x, y, -1,  1) as u8)
        .wrapping_add(self.neighbor_at_wrapping(x, y,  0,  1) as u8)
        .wrapping_add(self.neighbor_at_wrapping(x, y,  1,  1) as u8)
    }
    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Updates the Cell at the 2D-Coordinate.\
    /// Wraps if the `wraparound` flag is set.
    fn update_cell(&mut self, x: usize, y: usize) {
        if self.wraparound {
            self.update_cell_wrapping(x, y)
        } else {
            self.update_cell_checked(x, y)
        }
    }

    /// Updates the Cell at the 2D-Coordinate, excluding adjacent Cells that are out-of-bounds.
    fn update_cell_checked(&mut self, x: usize, y: usize) {
        let alive = self.read(x, y);
        let neighbors = self.neighbors(x, y);

        let next = Self::next_state(alive, neighbors);
        self.write(x, y, next);
    }

    /// Updates the Cell at the 2D-Coordinate.
    /// # Safety
    /// `x` and `y` must be in-bounds.\
    /// All adjacent Cells must be in-bounds.
    unsafe fn update_cell_unchecked(&mut self, x: usize, y: usize) {
        let alive = unsafe { self.read_unchecked(x, y) };
        let neighbors = unsafe { self.neighbors_unchecked(x, y) };

        let next = Self::next_state(alive, neighbors);
        unsafe { self.write_unchecked(x, y, next) };
    }

    /// Updates the Cell at the 2D-Coordinate.\
    /// Counts neighboring Cells, wrapping around the edges of the Board.
    fn update_cell_wrapping(&mut self, x: usize, y: usize) {
        let alive = self.read(x, y);
        let neighbors = self.neighbors_wrapping(x, y);

        let next = Self::next_state(alive, neighbors);
        unsafe { self.write_unchecked(x, y, next) };
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Updates all the Cells in the Board, then Swaps the Buffers.
    pub fn update(&mut self) {
        if Self::CHECKED {
            self.update_checked();
        } else {
            self.update_edges();
            self.update_inner();
        }

        self.swap_buffers();
    }

    /// Swaps the Read and Write Buffers.
    fn swap_buffers(&mut self) {
        core::mem::swap(&mut self.r_cells, &mut self.w_cells);
    }

    /// Updates all the Cells in the Board, with extra bounds-checking.
    fn update_checked(&mut self) {
        for y in self.y_range() {
            for x in self.x_range() {
                self.update_cell(x, y);
            }
        }
    }

    /// Updates all the Cells on the edges of the Board.
    fn update_edges(&mut self) {
        for x in self.x_range() {
            self.update_cell(x, 0);
        }

        for y in self.y_range_inner() {
            self.update_cell(0, y);
            self.update_cell(self.width - 1, y);
        }

        for x in self.x_range() {
            self.update_cell(x, self.height - 1);
        }
    }

    /// Updates all the Cells on the inside of the Board.
    fn update_inner(&mut self) {
        for y in self.y_range_inner() {
            for x in self.x_range_inner() {
                unsafe { self.update_cell_unchecked(x, y) };
            }
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Renders the Board-state to the Texture.
    pub fn render(&mut self) {
        let input = self.r_cells.as_slice();
        let output = self.image.as_mut_slice();
        assert_eq!(input.len(), output.len());

        let length = input.len();

        for index in 0..length {
            let cell = unsafe { *input.get_unchecked(index) };
            let pixel = unsafe { output.get_unchecked_mut(index) };

            let color = if cell {
                RGBA::rgba32(0xFFFFFFFF)
            } else {
                RGBA::rgba32(0xFF000000)
            };

            *pixel = color;
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Overwrites the Cell at the 2D-Coordinate `(x, y)`.\
    /// Does nothing if not in-bounds.
    pub fn overwrite(&mut self, x: usize, y: usize, val: bool) {
        if self.is_inbounds(x, y) {
            let index = unsafe { self.index_of(x, y) };
            self.r_cells[index] = val;
        }
    }

    /// Clears the Board-state.
    pub fn clear(&mut self) {
        self.r_cells.fill(false);
    }

    /// Randomizes the Board-state.
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        rng.fill(self.r_cells.as_mut_slice());
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //
}

// ================================================================================================================================ //
