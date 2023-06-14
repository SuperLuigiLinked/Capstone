/*
 *  Crate: Wyn
 * Module: Cocoa - Screen
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::event_loop::EventLoop;
use super::types::Rect;

// ================================================================================================================================ //

/// Native OS Representation for Screens.
pub type NativeScreen = *mut sys::NSScreen;

/// Wrapper type that holds Info about Screens.
pub struct Screen(pub NativeScreen);

// ================================================================================================================================ //

impl Screen {
    /// Returns the bounding rectangle of the Screen.
    pub fn rect(&self, _events: &EventLoop) -> Rect {
        let ns_rect = unsafe { sys::ns_screen::frame(self.0) };
        Rect::from(ns_rect)
    }

    /// Returns the name of the Screen.
    pub fn name(&self, _events: &EventLoop) -> String {
        unsafe {
            let ns_str = sys::ns_screen::localizedName(self.0);
            assert!(!ns_str.is_null());

            let cstr = sys::ns_string::UTF8String(ns_str);
            assert!(!cstr.is_null());
            let len = sys::strlen(cstr);

            let slice = std::slice::from_raw_parts(cstr as *const u8, len);
            String::from_utf8_lossy(slice).to_string()
        }
    }
}

// ================================================================================================================================ //

impl Screen {
    /// Returns the Primary Screen.
    pub fn primary(_events: &EventLoop) -> Self {
        let ns_screen = unsafe { sys::ns_screen::mainScreen() };
        Self(ns_screen)
    }

    /// Collects a list of all the available Screens.
    pub fn collect(_events: &EventLoop) -> Vec<Self> {
        let mut vec = Vec::new();

        let ns_array = unsafe { sys::ns_screen::screens() };
        let len = unsafe { sys::ns_array::count(ns_array) };

        vec.reserve(len as usize);
        for i in 0..len {
            let ns_screen = unsafe { sys::ns_array::objectAtIndex(ns_array, i) };
            let display = Self(ns_screen);
            vec.push(display);
        }

        vec
    }
}

// ================================================================================================================================ //
