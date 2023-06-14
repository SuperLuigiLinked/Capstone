/*
 *   Crate: RGE (Rust Game Engine)
 *  Author: Eric Grandizio <grandizioe@duq.edu>
 * License: Creative Commons Zero v1.0 Universal
 */

//! # Rust Game Engine (RGE)
//!
//! A library that makes writing a game simple.
//!
//! Supports 2D Graphics, such as Sprites, Tilemaps, and Shapes.

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

pub(crate) mod settings;
pub use self::settings::*;

pub(crate) mod engine;
pub use self::engine::*;

pub(crate) mod timer;
pub use self::timer::*;

pub mod inputs;

pub(crate) mod texture;
pub use self::texture::*;

pub(crate) mod vulkan;

// ================================================================================================================================ //
