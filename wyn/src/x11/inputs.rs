/*
 *  Crate: Wyn
 * Module: X11 - Inputs
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// A button on a mouse.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct MouseButton(pub u32);

/// A key on a keyboard.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct KeyCode(pub u32);

// ================================================================================================================================ //
