/*
 *  Crate: Wyn
 * Module: Win32 - Events
 */

//! Provides Event Handler callback functions.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::event_loop::EventLoop;
use super::inputs::{KeyCode, MouseButton};
use super::types::Point;
use super::window::WindowHandle;

// ================================================================================================================================ //

#[allow(unused_variables)]
/// Trait for responding to events.
///
/// The `self` parameter must be an immutable reference for the following reasons:
/// * **Thread-Safety:** The trait object is allowed to be shared across multiple threads.
/// * **Reentrancy:** Some platforms (e.g. Windows) use re-entrant message callbacks.
///
/// Because Rust does not allow multiple `&mut` to the same object simultaneously, passing `&mut self` could lead to Undefined Behavior.
pub trait EventHandler: Sync {
    /// The Event Loop is about to start running.
    fn start(&self, events: &EventLoop) {}

    /// The Event Loop has stopped running.
    fn stop(&self, events: &EventLoop) {}

    /// A Window was opened, and its handle is available to use.
    fn window_open(&self, events: &EventLoop, handle: WindowHandle) {}

    /// A Window is about to close, and its handle must be given up.
    fn window_close(&self, events: &EventLoop, handle: WindowHandle) {}

    /// A Window needs its contents redrawn.
    fn window_redraw(&self, events: &EventLoop, handle: WindowHandle) {}

    /// A Window was repositioned (moved/resized).
    fn window_reposition(&self, events: &EventLoop, handle: WindowHandle) {}

    /// A Window is about to be shown or hidden.
    fn window_visibility(&self, events: &EventLoop, handle: WindowHandle, visible: bool) {}

    /// A Window was focused/unfocused.
    fn window_focus(&self, events: &EventLoop, handle: WindowHandle, focused: bool) {}

    /// A Mouse-Cursor was moved across the Window.
    fn cursor_move(&self, events: &EventLoop, handle: WindowHandle, point: Point) {}

    /// A Scroll-Wheel was scrolled horizontally/vertically on the Window.
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

    /// A Key was pressed/released in the Window.
    fn key_press(&self, events: &EventLoop, handle: WindowHandle, keycode: KeyCode, pressed: bool) {
    }

    /// A Character was input in the Window.
    fn character_input(&self, events: &EventLoop, handle: WindowHandle, character: char) {}
}

// ================================================================================================================================ //
