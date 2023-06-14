/*
 *  Crate: Wyn
 * Module: Common
 */

//! Internally-used items shared across different platforms.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

pub(crate) use core::fmt::{Debug, Display};

pub(crate) use ::defer::defer;

pub(crate) use ::cfg_if::cfg_if;

// ================================================================================================================================ //

pub(crate) mod nonzero;
pub(crate) use self::nonzero::{NonNull, NonZero};

pub(crate) mod types;

pub mod tasks;

// ================================================================================================================================ //

/// A type alias for `Box`ed `Panic`s.\
/// Useful for catching/resuming panic-unwinding across FFI-boundaries.
pub(crate) type BoxedPanic = Box<dyn std::any::Any + Send + 'static>;

// ================================================================================================================================ //
