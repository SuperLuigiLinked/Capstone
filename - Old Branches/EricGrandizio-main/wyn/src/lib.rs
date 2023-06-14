/*
 *   Crate: Wyn
 *  Author: Eric Grandizio <grandizioe@duq.edu>
 * License: Creative Commons Zero v1.0 Universal
 */

//! ***Wyn*** is a cross-platform windowing library targeting Windows/Linux/MacOS, written entirely in Rust.
//!
//! It wraps the underlying platforms closely, providing a single common API for differing platforms, with minimal overhead.
//!
//! ### Features
//! * Manipulate windows on the desktop.
//! * Run native platform event/message loops.
//! * Receive user-inputs (such as mouse, keyboard, and controller inputs).
//! * Query the state of windows and monitors.

// ================================================================================================================================ //

#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::bare_urls)]
#![warn(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

// ================================================================================================================================ //

#[allow(unused_imports)]
#[macro_use]
extern crate gui_sys as sys;

#[allow(unused_imports)]
use sys::common::utils::*;

// -------------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
pub(crate) mod common;
pub use self::common::*;

// -------------------------------------------------------------------------------------------------------------------------------- //

cfg_if! {
    if #[cfg(target_os = "windows")]
    {
        mod win32;
        pub use self::win32::*;
        pub use self::win32::types;
    }
    else if #[cfg(target_os = "linux")]
    {
        mod x11;
        pub use self::x11::*;
        pub use self::x11::types;
    }
    else if #[cfg(target_os = "macos")]
    {
        mod cocoa;
        pub use self::cocoa::*;
        pub use self::cocoa::types;
    }
}

// ================================================================================================================================ //
