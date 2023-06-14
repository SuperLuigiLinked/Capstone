/*
 *   Crate: GUI-Sys
 *  Author: Eric Grandizio <grandizioe@duq.edu>
 * License: Creative Commons Zero v1.0 Universal
 */

//! Re-exported items from dependencies.
//!
//! This crate re-exports functions, types, macros, and constants from its dependencies,\
//! providing access via a short, unified module name.
//!
//! Each item re-exported in this crate will also link to its Official Documentation.\
//! Thus, this crate doubles as book-keeping for the items used from dependencies.

// ================================================================================================================================ //

#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::missing_safety_doc)]

// ================================================================================================================================ //

#[macro_use]
pub mod common;
pub use self::common::*;

// ================================================================================================================================ //
cfg_if::cfg_if! {
// -------------------------------------------------------------------------------------------------------------------------------- //
if #[cfg(target_os = "windows")]
{
    #[macro_use]
    pub mod windows;
    pub use self::windows::*;
}
// -------------------------------------------------------------------------------------------------------------------------------- //
else if #[cfg(target_os = "linux")]
{
    #[macro_use]
    pub mod linux;
    pub use self::linux::*;
}
// -------------------------------------------------------------------------------------------------------------------------------- //
else if #[cfg(target_os = "macos")]
{
    #[macro_use]
    pub mod macos;
    pub use self::macos::*;
}
// -------------------------------------------------------------------------------------------------------------------------------- //
}
// ================================================================================================================================ //
