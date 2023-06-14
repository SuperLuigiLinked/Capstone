/*
 *  Crate: GUI-Sys
 * Module: Common - Libc
 */

//! Common Libc bindings.
//!
//! # Dependencies
//! * <https://crates.io/crates/libc>
//!
//! # Documentation
//! * <https://en.cppreference.com/w/c>

// -------------------------------------------------------------------------------------------------------------------------------- //

use crate::common::c_types::*;

// ================================================================================================================================ //
// Functions
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://en.cppreference.com/w/c/memory/malloc>
pub use ::libc::malloc;

/// <https://en.cppreference.com/w/c/memory/free>
pub use ::libc::free;

/// <https://en.cppreference.com/w/c/string/byte/strlen>
pub use ::libc::strlen;

/// <https://en.cppreference.com/w/c/string/byte/strcmp>
pub use ::libc::strcmp;

// ================================================================================================================================ //
