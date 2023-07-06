/*
 *  Crate: Wyn
 * Module: X11 - Screen
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::event_loop::EventLoop;
use super::types::Rect;

// ================================================================================================================================ //

/// Native OS Representation for Screens.
pub type NativeScreen = *mut sys::xcb_screen_t;

/// Wrapper type that holds Info about Screens/
pub struct Screen(pub NativeScreen);

// ================================================================================================================================ //

impl Screen {
    /// Returns the bounding rectangle of the Screen.
    pub fn rect(&self, _events: &EventLoop) -> Rect {
        todo!();
    }

    /// Returns the name of the Screen.
    pub fn name(&self, _events: &EventLoop) -> String {
        todo!();
    }
}

// ================================================================================================================================ //

impl Screen {
    /// Returns the Primary Screen.
    pub fn primary(_events: &EventLoop) -> Screen {
        todo!();
    }

    /// Collects a list of all the available Screens.
    pub fn collect(_events: &EventLoop) -> Vec<Screen> {
        todo!();
    }
}

// ================================================================================================================================ //
