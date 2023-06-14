/*
 *  Crate: GUI-Sys
 * Module: Common
 */

//! Common dependencies.

// ================================================================================================================================ //

pub mod c_types;
pub use self::c_types::*;

// -------------------------------------------------------------------------------------------------------------------------------- //

#[macro_use]
pub mod utils;
pub use self::utils::*;

// -------------------------------------------------------------------------------------------------------------------------------- //

#[macro_use]
pub mod libc;
pub use self::libc::*;

// ================================================================================================================================ //