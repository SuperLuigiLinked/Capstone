/*
 *  Crate: RGE
 * Module: Engine
 */

//! Game Engine functionality.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use crate::vulkan::Vulkan;

use wyn::{
    event_loop::EventLoop,
    events::EventHandler,
    screen::Screen,
    window::{Window, WindowHandle},
};

pub use wyn::types::{Coord, Extent, Point, Rect, Size};

use std::sync::RwLock;
use std::{
    ops::{Deref, DerefMut},
    sync::Barrier,
};

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

    /// Waitable VSYNC barrier, for synchronizing Update and Render threads.
    vsync_barrier: Barrier,

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

    /// User-control for Window.
    pub window: WindowSettings,

    /// Settings for the Renderer.
    pub render: RenderSettings,

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
        let vk = Vulkan::new();

        let state = EngineState {
            vk,
            wyn_window,
            render,
            window,
            timer,
        };

        let state = RwLock::new((game, state));

        let vsync_barrier = Barrier::new(2);

        Self {
            state,
            vsync_barrier,
            init_settings,
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Locks the Engine State and runs the Callback.
    fn write_state<T>(&self, func: impl FnOnce(&mut dyn Game, &mut EngineState) -> T) -> T {
        let mut lock = self.state.write().unwrap();
        let (game, state) = lock.deref_mut();
        func(*game, state)
    }

    /// Locks the Engine State and runs the Callback.
    fn read_state<T>(&self, func: impl FnOnce(&dyn Game, &EngineState) -> T) -> T {
        let lock = self.state.read().unwrap();
        let game = lock.0.deref();
        let state = &lock.1;
        func(game, state)
    }

    /// Tries to Lock the Engine State and run the Callback.
    #[allow(unused)]
    fn try_write_state<T>(
        &self,
        func: impl FnOnce(&mut dyn Game, &mut EngineState) -> T,
    ) -> Option<T> {
        let res = self.state.try_write();
        if let Ok(mut lock) = res {
            let (game, state) = lock.deref_mut();
            Some(func(*game, state))
        } else {
            None
        }
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
    }
}

// ================================================================================================================================ //

impl<'a> EventHandler for GameEngine<'a> {
    fn start(&self, events: &EventLoop) {
        let window = Window::open(events).unwrap();

        self.write_state(move |_game, state| {
            let vsync = state.timer.vsync();
            state.vk.create_surface(window.handle(), vsync);
            state.wyn_window = Some(window);

            state.window.name = String::from("RGE");
        });

        self.read_state(|_game, state| {
            if let Some(window) = state.wyn_window.as_ref() {
                self::write_window(events, window, &state.window);

                let screen = Screen::primary(events);
                let sc_rect = screen.rect(events);

                let center = sc_rect.center();
                let size = Size::new(self.init_settings.width, self.init_settings.height);
                let wn_rect = Rect::new_centered(center, size);

                window.reposition_content(events, wn_rect).unwrap();
                window.focus(events).unwrap();
            }
        });
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
        let (should_quit, should_vsync) = self.write_state(|game, state| {
            if let Some(window) = state.wyn_window.as_ref() {
                if window.handle() != handle {
                    return (false, false);
                }
            } else {
                return (false, false);
            };

            let should_quit = {
                let window = state.wyn_window.as_ref().unwrap();

                state.window = self::read_window(events, window);
                !state.internal_render(events, game)
            };

            let should_vsync = !should_quit && {
                let window = state.wyn_window.as_ref().unwrap();

                self::write_window(events, window, &state.window);
                state.timer.vsync_bounded()
            };

            (should_quit, should_vsync)
        });

        if should_vsync {
            self.vsync_barrier.wait();
        }

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
        assert!(events.await_startup());
        let _defer = ::defer::defer(|| self.start_shutdown(events));

        self.write_state(|_game, state| {
            state.timer.toggle_vsync(self.init_settings.vsync);
            state.timer.reset_fps(self.init_settings.fps);
        });

        while events.is_running() {
            {
                let should_vsync = self.write_state(|_game, state| {
                    if state.timer.vsync_bounded() {
                        if let Some(window) = state.wyn_window.as_ref() {
                            window.request_redraw(events);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                if should_vsync {
                    self.vsync_barrier.wait();
                }
            }

            let (should_quit, next_tick) = self.write_state(|game, state| {
                state.timer.update();

                let should_quit = !state.internal_update(events, game);
                let next_tick = state.timer.next_tick();

                (should_quit, next_tick)
            });

            if should_quit {
                break;
            }

            FrameTimer::sync(next_tick);
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
}

/// Reads the Window-Settings of a Window.
fn read_window(events: &EventLoop, window: &Window) -> WindowSettings {
    let name = window.name(events).unwrap();

    WindowSettings { name }
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

        let Some(_window) = self.wyn_window.as_ref()
        else {
            return false;
        };

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
