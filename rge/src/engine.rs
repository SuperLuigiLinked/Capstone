/*
 *  Crate: RGE
 * Module: Engine
 */

//! Game Engine functionality.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use crate::{inputs::*, vulkan::Vulkan};

use wyn::{
    event_loop::EventLoop,
    events::EventHandler,
    screen::Screen,
    window::{Window, WindowHandle, WindowStyle},
};
pub use wyn::{
    screen::ScreenInfo,
    types::{Coord, Extent, Point, Rect, Size},
};

use std::{
    ops::{Deref, DerefMut},
    sync::{Condvar, Mutex},
};

use parking_lot::{RwLock, RwLockWriteGuard};

// ================================================================================================================================ //

/// A trait that represents an App/Game.
pub trait Game: Sync {
    /// The core game logic should be run.
    /// Return `false` or `panic` to stop the Game Loop.
    fn update(&mut self, engine: &mut EngineState) -> bool;

    /// The render logic should be run.
    /// Return `false` or `panic` to stop the Game Loop.
    fn render(&mut self, engine: &mut EngineState) -> bool;
}

// ================================================================================================================================ //

/// Runner for Game Loops.
pub struct GameEngine<'a> {
    /// Internal shared state, wrapped in a Reader-Writer Lock.
    state: RwLock<(&'a mut dyn Game, EngineState)>,

    /// Condition Variable for awaiting/signaling VSYNC.
    vsync_condvar: Condvar,

    /// Mutex holding the Number of the Last Update that was rendered.
    vsync_mutex: Mutex<usize>,

    /// The queued input-events.
    inputs: Mutex<Inputs>,

    /// Initial settings.
    init_settings: GameEngineSettings,
}
unsafe impl<'a> Sync for GameEngine<'a> {}

/// State wrapped by Game Engine.
pub struct EngineState {
    /// Vulkan Renderer.
    pub(crate) vk: Vulkan,

    /// The visible Window on the Desktop.
    pub(crate) wyn_window: Option<Window>,

    /// User-control for Window.\
    /// The user may change these during the Render callback.
    pub window: WindowSettings,

    /// Settings for the Renderer.\
    /// The user may change these during the Render callback.
    pub render: RenderSettings,

    /// Information about the connected Screens/Monitors.
    pub screens: Vec<ScreenInfo>,

    /// The Inputs that were queued between Update calls.
    pub inputs: Inputs,

    /// Timer for scheduling Update-calls.
    pub timer: FrameTimer,
}

impl<'a> GameEngine<'a> {
    /// Constructs a new `GameEngine` object.
    pub fn new(game: &'a mut dyn Game, init_settings: GameEngineSettings) -> Self {
        let timer = FrameTimer::new(init_settings.fps, init_settings.vsync);

        let wyn_window = None;
        let window = Default::default();
        let render = Default::default();
        let screens = Default::default();
        let inputs = Default::default();

        let vk = Vulkan::new();

        let state = EngineState {
            vk,
            wyn_window,
            render,
            window,
            screens,
            inputs,
            timer,
        };

        let state = RwLock::new((game, state));

        let vsync_condvar = Condvar::new();
        let vsync_mutex = Mutex::new(0);

        let inputs = Mutex::new(Inputs::default());

        Self {
            state,
            vsync_condvar,
            vsync_mutex,
            inputs,
            init_settings,
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Locks the Engine State and runs the Callback.
    fn write_state<T>(&self, func: impl FnOnce(&mut dyn Game, &mut EngineState) -> T) -> T {
        let mut lock = self.state.write();
        let (game, state) = lock.deref_mut();
        func(*game, state)
    }

    /// Locks the Engine State and runs the Callback.
    fn read_state<T>(&self, func: impl FnOnce(&dyn Game, &EngineState) -> T) -> T {
        let lock = self.state.read_recursive();
        let (game, state) = (lock.0.deref(), &lock.1);
        func(game, state)
    }

    /// Locks the Engine State and runs the Write Callback with a Writer Lock, then the Read Callback with a Reader Lock.
    fn write_read_state<T1, T2>(
        &self,
        write_func: impl FnOnce(&mut dyn Game, &mut EngineState) -> T1,
        read_func: impl FnOnce(&dyn Game, &EngineState, T1) -> T2,
    ) -> T2 {
        let mut write_lock = self.state.write();
        let (game, state) = write_lock.deref_mut();
        let res = write_func(*game, state);

        let read_lock = RwLockWriteGuard::downgrade(write_lock);
        let (game, state) = (read_lock.0.deref(), &read_lock.1);
        read_func(game, state, res)
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Starts the shutdown process.
    fn start_shutdown(&self, events: &EventLoop) {
        if let Some(window) = self.write_state(|_game, state| {
            if let Some(window) = state.wyn_window.take() {
                state.vk.destroy_surface(window.handle());
                Some(window)
            } else {
                None
            }
        }) {
            window.close(events).unwrap();
            events.request_stop();
        }

        self.signal_render(usize::MAX);
    }
}

// ================================================================================================================================ //

impl<'a> EventHandler for GameEngine<'a> {
    fn start(&self, events: &EventLoop) {
        let window = Window::open(events).unwrap();

        self.write_read_state(
            move |_game, state| {
                let vsync = state.timer.vsync();
                state.vk.create_surface(window.handle(), vsync);
                state.wyn_window = Some(window);

                state.window.name = String::from("RGE");
            },
            |_game, state, _| {
                if let Some(window) = state.wyn_window.as_ref() {
                    self::write_window(events, window, &state.window);

                    let screen = Screen::primary(events);
                    let sc_rect = screen.rect(events);

                    let center = sc_rect.center();
                    let size = Size::new(self.init_settings.width, self.init_settings.height);
                    let wn_rect = Rect::new_centered(center, size);

                    window.reposition_content(events, wn_rect).unwrap();

                    if self.init_settings.fullscreen || (sc_rect == wn_rect) {
                        let _ = window.fullscreen(events);
                    }

                    window.focus(events).unwrap();
                }
            },
        );
    }

    fn stop(&self, events: &EventLoop) {
        self.start_shutdown(events);
    }

    fn window_reposition(&self, events: &EventLoop, handle: WindowHandle) {
        self.read_state(|_game, state| {
            if let Some(window) = state.wyn_window.as_ref() {
                if window.handle() == handle {
                    window.request_redraw(events);
                }
            }
        });
    }

    fn window_redraw(&self, events: &EventLoop, handle: WindowHandle) {
        let should_quit = self.write_read_state(
            |game, state| {
                if let Some(window) = state.wyn_window.as_ref() {
                    if window.handle() == handle {
                        state.window = self::read_window(events, window);
                    } else {
                        return true;
                    }
                } else {
                    return true;
                }

                let should_quit = !state.internal_render(events, game);

                let this_update = state.render.updates;
                self.signal_render(this_update);

                should_quit
            },
            |_game, state, should_quit| {
                should_quit || {
                    if let Some(window) = state.wyn_window.as_ref() {
                        self::write_window(events, window, &state.window);
                        false
                    } else {
                        true
                    }
                }
            },
        );

        if should_quit {
            self.start_shutdown(events);
        }
    }

    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {
        let should_quit = self.write_state(|_game, state| {
            if let Some(window) = state.wyn_window.as_ref() {
                window.handle() == handle
            } else {
                false
            }
        });

        if should_quit {
            self.start_shutdown(events);
        }
    }

    // ---------------------------------------------------------------- //

    fn window_focus(&self, _events: &EventLoop, _handle: WindowHandle, focused: bool) {
        let input = Input::WindowFocus { focused };
        let mut inputs = self.inputs.lock().unwrap();
        inputs.events.push(input);
    }

    fn cursor_move(&self, _events: &EventLoop, _handle: WindowHandle, point: Point) {
        let input = Input::CursorMove { point };
        let mut inputs = self.inputs.lock().unwrap();
        inputs.events.push(input);
    }

    fn scroll_wheel(&self, _events: &EventLoop, _handle: WindowHandle, delta_x: f64, delta_y: f64) {
        let input = Input::ScrollWheel { delta_x, delta_y };
        let mut inputs = self.inputs.lock().unwrap();
        inputs.events.push(input);
    }

    fn button_press(
        &self,
        _events: &EventLoop,
        _handle: WindowHandle,
        button: MouseButton,
        pressed: bool,
    ) {
        let input = Input::ButtonPress { button, pressed };
        let mut inputs = self.inputs.lock().unwrap();
        inputs.events.push(input);
    }

    fn key_press(
        &self,
        _events: &EventLoop,
        _handle: WindowHandle,
        keycode: KeyCode,
        pressed: bool,
    ) {
        let input = Input::KeyPress { keycode, pressed };
        let mut inputs = self.inputs.lock().unwrap();
        inputs.events.push(input);
    }

    fn character_input(&self, _events: &EventLoop, _handle: WindowHandle, character: char) {
        let input = Input::CharacterInput { character };
        let mut inputs = self.inputs.lock().unwrap();
        inputs.events.push(input);
    }
}

// ================================================================================================================================ //

impl<'a> GameEngine<'a> {
    /// Runs the Game Loop.
    pub fn run(&self) {
        let events = EventLoop::new(self).unwrap();

        std::thread::scope(|scope| {
            let thread = scope.spawn(|| self.update_thread(&events));
            events.run();
            thread.join().unwrap();
        });
    }

    /// Logic to handle Updates at a regular interval.
    fn update_thread(&self, events: &EventLoop) {
        if !events.await_startup() {
            return;
        }
        let _defer = ::defer::defer(|| {
            self.start_shutdown(events);
        });

        self.write_state(|_game, state| {
            state.timer.toggle_vsync(self.init_settings.vsync);
            state.timer.reset_fps(self.init_settings.fps);
            state.timer.reset_epoch(std::time::Instant::now());
        });

        while events.is_running() {
            let (should_quit, next_tick, this_update) = self.write_state(|game, state| {
                state.timer.update();

                state.inputs = {
                    let mut lock = self.inputs.lock().unwrap();

                    let screens = Screen::collect(events)
                        .into_iter()
                        .map(|screen| screen.info(events))
                        .collect();

                    let xinput: XInputControllers = XInputController::collect(events).unwrap();
                    let events: Vec<Input> = lock.events.clone();
                    lock.events.clear();

                    Inputs {
                        events,
                        xinput,
                        screens,
                    }
                };

                let should_quit = !state.internal_update(events, game);
                let next_tick = state.timer.next_tick();
                let this_update = state.render.updates;

                (should_quit, next_tick, this_update)
            });

            if should_quit {
                break;
            }

            if !self.await_render(this_update) {
                break;
            }

            FrameTimer::sync(next_tick);
        }
    }

    /// Waits until the Rendered-Update Count is greater-than or equal-to count.
    fn await_render(&self, count: usize) -> bool {
        if let Ok(rendered) = self.vsync_mutex.lock() {
            if *rendered < count {
                let _ = self
                    .vsync_condvar
                    .wait_while(rendered, |rendered| *rendered < count);
            }

            true
        } else {
            false
        }
    }

    /// Writes the Rendered-Update Count and notifies the Update Thread.
    fn signal_render(&self, count: usize) {
        if let Ok(mut lock) = self.vsync_mutex.lock() {
            *lock = count;
            self.vsync_condvar.notify_one();
        }
    }
}

// ================================================================================================================================ //

/// Updates the Window-Settings of a Window.
fn write_window(events: &EventLoop, window: &Window, settings: &WindowSettings) {
    let old_name = window.name(events).unwrap();
    let new_name = &settings.name;
    if new_name != &old_name {
        window.rename(events, new_name).unwrap();
    }

    let old_rect = window.content_rect(events).unwrap();
    let new_rect = settings.rect;
    if new_rect != old_rect {
        window.reposition_content(events, new_rect).unwrap();
    }

    let old_fullscreen = window.is_fullscreen(events);
    let new_fullscreen = settings.fullscreen;
    if new_fullscreen != old_fullscreen {
        if new_fullscreen {
            window.set_style(events, WindowStyle::Borderless).unwrap();
            window.fullscreen(events).unwrap();
        } else {
            window.restore(events).unwrap();
            window.set_style(events, WindowStyle::Captioned).unwrap();
        }
    }
}

/// Reads the Window-Settings of a Window.
fn read_window(events: &EventLoop, window: &Window) -> WindowSettings {
    let name = window.name(events).unwrap();
    let rect = window.content_rect(events).unwrap();
    let fullscreen = window.is_fullscreen(events);

    WindowSettings {
        name,
        rect,
        fullscreen,
    }
}

// ================================================================================================================================ //

impl EngineState {
    /// Runs the Update callback.
    /// Returns `false` or `panics` if the Event Loop should stop.
    fn internal_update(&mut self, events: &EventLoop, game: &mut dyn Game) -> bool {
        if self.wyn_window.is_none() {
            return false;
        }

        {
            self.render.updates += 1;
        }

        if !game.update(self) {
            return false;
        }

        if let Some(window) = self.wyn_window.as_ref() {
            window.request_redraw(events);
        }

        true
    }

    /// Runs the Render callback.
    /// Returns `false` or `panics` if the Event Loop should stop.
    fn internal_render(&mut self, _events: &EventLoop, game: &mut dyn Game) -> bool {
        if self.wyn_window.is_none() {
            return false;
        }

        if !game.render(self) {
            return false;
        }

        // ---------------------------------------------------------------- //

        if self.wyn_window.is_none() {
            return false;
        }

        let Some(vk_window) = (unsafe { self.vk.renderable_mut() })
        else {
            return false;
        };

        {
            self.render.vsync = self.timer.vsync();
            self.render.fps = self.timer.fps();
            self.render.renders += 1;
        }

        {
            //let t1 = std::time::Instant::now();

            vk_window.render(&self.render);

            // let t2 = std::time::Instant::now();
            // let elapsed = t2.saturating_duration_since(t1);
            // let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
            // eprintln!("ELAPSED: {}", elapsed_ms);
        }

        // ---------------------------------------------------------------- //

        true
    }
}

// ================================================================================================================================ //

impl EngineState {
    /// Updates the current Texture Atlas.
    pub fn update_atlas(&mut self, texture: &Texture) {
        self.vk.update_atlas(texture);
    }
}

// ================================================================================================================================ //
