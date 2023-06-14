/*
 *  Crate: Wyn
 * Module: Win32
 */

//! The implementation of Wyn for Windows, using the Win32 backend.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

pub(crate) mod log;

#[macro_use]
pub mod errors;

pub mod events;

pub(crate) mod event_data;
pub mod event_loop;

pub mod screen;

pub mod window;

pub mod types;

pub mod inputs;

// ================================================================================================================================ //
