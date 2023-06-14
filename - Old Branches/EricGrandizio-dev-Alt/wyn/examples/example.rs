/*
 *   Crate: Wyn
 * Example: Example.rs
 */

#![windows_subsystem = "windows"]

// ================================================================================================================================ //

extern crate wyn;
use std::sync::RwLock;

use wyn::event_loop::EventLoop;
use wyn::events::EventHandler;
use wyn::inputs::{KeyCode, MouseButton};
use wyn::screen::Screen;
use wyn::types::{Point, Rect};
use wyn::window::{Window, WindowHandle};

// ================================================================================================================================ //

pub fn main() {
    println!("[MAIN BEGIN]");
    {
        let app = App::new();
        app.run();
    }
    println!("[MAIN END]");
}

// ================================================================================================================================ //

struct App {
    window: RwOpt<Window>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl App {
    pub fn new() -> Self {
        Self {
            window: RwOpt::new(None),
        }
    }

    pub fn run(&self) {
        let events = EventLoop::new(self).unwrap();
        events.run();
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventHandler for App {
    fn start(&self, events: &EventLoop) {
        println!("[START]");

        {
            {
                let disp = Screen::primary(events);
                let name = disp.name(events);
                let rect = disp.rect(events);
                eprintln!("MAIN: {name:?} | {rect:?}");
            }

            let disps = Screen::collect(events);
            for (i, disp) in disps.iter().enumerate() {
                let name = disp.name(events);
                let rect = disp.rect(events);
                eprintln!("DISPLAY[{i}]: {name:?} | {rect:?}");
            }
        }

        if true {
            std::process::exit(0);
        }

        eprintln!("WINDOW...");
        let window = Window::open(events).unwrap();
        self.window.set(Some(window));

        {
            let window2 = Window::open(events).unwrap();
            window2.rename(events, "Window 2").unwrap();
            window2.show(events).unwrap();
        }

        self.window.read(|window| {
            eprintln!("FOCUS...");
            window.focus(events).unwrap();

            eprintln!("RENAME...");
            window.rename(events, "Hello, World!").unwrap();

            let title = window.name(events).unwrap();
            eprintln!("TITLE: [{}] {title:?}", title.len());

            let cr = window.content_rect(events);
            let br = window.border_rect(events);
            eprintln!("CR: {cr:?}");
            eprintln!("BR: {br:?}");

            if true {
                eprintln!("RESIZE BORDER...");
                window
                    .reposition_border(events, Rect::new(200.0, 100.0, 320.0, 240.0))
                    .unwrap();
            } else {
                eprintln!("RESIZE CONTENT...");
                window
                    .reposition_content(events, Rect::new(200.0, 100.0, 320.0, 240.0))
                    .unwrap();
            }

            let cr = window.content_rect(events);
            let br = window.border_rect(events);
            eprintln!("CR: {cr:?}");
            eprintln!("BR: {br:?}");
        });
    }

    fn stop(&self, _events: &EventLoop) {
        println!("[STOP]");
    }

    fn window_open(&self, _events: &EventLoop, handle: WindowHandle) {
        println!("[WINDOW OPEN : {handle:?}]");
    }

    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {
        println!("[WINDOW CLOSE : {handle:?}]");

        let should_quit = self
            .window
            .read(|window| window.handle() == handle)
            .unwrap_or_default();

        if should_quit {
            let _ = self.window.take();
            events.request_stop();
        }
    }

    fn window_reposition(&self, events: &EventLoop, handle: WindowHandle) {
        println!("[WINDOW REPOSITION : {handle:?}]");

        self.window.read(|window| {
            if window.handle() == handle {
                let cr = window.content_rect(events);
                let br = window.border_rect(events);
                println!("CONTENT: {cr:?}");
                println!(" BORDER: {br:?}");
            }
        });
    }

    fn window_focus(&self, _events: &EventLoop, handle: WindowHandle, focused: bool) {
        if focused {
            println!("[WINDOW FOCUS : {handle:?}]");
        } else {
            println!("[WINDOW UNFOCUS : {handle:?}]");
        }
    }

    fn cursor_move(&self, _events: &EventLoop, handle: WindowHandle, point: Point) {
        println!("[CURSOR MOVE : {handle:?}] : {point:?}");
    }

    fn scroll_wheel(&self, _events: &EventLoop, handle: WindowHandle, delta_x: f64, delta_y: f64) {
        println!("[SCROLL WHEEL : {handle:?}] : ({delta_x:?} , {delta_y:?})");
    }

    fn button_press(
        &self,
        _events: &EventLoop,
        handle: WindowHandle,
        button: MouseButton,
        pressed: bool,
    ) {
        if pressed {
            println!("[BUTTON PRESS   : {handle:?}] : {button:?}");
        } else {
            println!("[BUTTON RELEASE : {handle:?}] : {button:?}");
        }
    }

    fn key_press(
        &self,
        _events: &EventLoop,
        handle: WindowHandle,
        keycode: KeyCode,
        pressed: bool,
    ) {
        if pressed {
            println!("[KEY PRESS   : {handle:?}] : {keycode:?}");
        } else {
            println!("[KEY RELEASE : {handle:?}] : {keycode:?}");
        }
    }
}

// ================================================================================================================================ //

#[repr(transparent)]
pub struct RwOpt<T>(pub RwLock<Option<T>>);

impl<T> RwOpt<T> {
    pub const fn new(opt: Option<T>) -> Self {
        Self(RwLock::new(opt))
    }

    pub fn write_opt<R>(&self, func: impl FnOnce(Option<&mut T>) -> R) -> R {
        let mut lock = self.0.write().unwrap();
        let opt = lock.as_mut();
        func(opt)
    }

    pub fn read_opt<R>(&self, func: impl FnOnce(Option<&T>) -> R) -> R {
        let lock = self.0.read().unwrap();
        let opt = lock.as_ref();
        func(opt)
    }

    pub fn write<R>(&self, func: impl FnOnce(&mut T) -> R) -> Option<R> {
        let mut lock = self.0.write().unwrap();
        let opt = lock.as_mut();
        opt.map(func)
    }

    pub fn read<R>(&self, func: impl FnOnce(&T) -> R) -> Option<R> {
        let lock = self.0.read().unwrap();
        let opt = lock.as_ref();
        opt.map(func)
    }

    pub fn take(&self) -> Option<T> {
        let mut lock = self.0.write().unwrap();
        lock.take()
    }

    pub fn set(&self, opt: Option<T>) {
        let mut lock = self.0.write().unwrap();
        *lock = opt;
    }
}

// ================================================================================================================================ //
