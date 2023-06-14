/*
 *  Crate: Wyn
 * Module: Cocoa - Events
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::event_loop::EventLoop;
use super::inputs::{KeyCode, MouseButton};
use super::types::Point;
use super::window::WindowHandle;

// ================================================================================================================================ //

#[allow(unused_variables)]
/// Callback functions for events.
pub trait EventHandler: Sync {
    /// The Event Loop has started.
    fn start(&self, events: &EventLoop) {}

    /// The Event Loop has stopped.
    fn stop(&self, events: &EventLoop) {}

    /// A Window was opened.
    fn window_open(&self, events: &EventLoop, handle: WindowHandle) {}

    /// A Window was closed.
    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {}

    /// A Window was repositioned (moved/resized).
    fn window_reposition(&self, events: &EventLoop, handle: WindowHandle) {}

    /// A Window was focused/unfocused.
    fn window_focus(&self, events: &EventLoop, handle: WindowHandle, focused: bool) {}

    /// A Mouse-Cursor was moved across the Window.
    fn cursor_move(&self, events: &EventLoop, handle: WindowHandle, point: Point) {}

    /// A Mouse-Wheel was scrolled horizontally/vertically on the Window.
    fn scroll_wheel(&self, events: &EventLoop, handle: WindowHandle, delta_x: f64, delta_y: f64) {}

    /// A Mouse-Button was pressed/released on the Window.
    fn button_press(
        &self,
        events: &EventLoop,
        handle: WindowHandle,
        button: MouseButton,
        pressed: bool,
    ) {
    }

    /// A Key was pressed/released on the Window.
    fn key_press(&self, events: &EventLoop, handle: WindowHandle, keycode: KeyCode, pressed: bool) {
    }
}

// ================================================================================================================================ //
