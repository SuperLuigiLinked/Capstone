/*
 *  Crate: Wyn
 * Module: Cocoa - Inputs
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Native OS Representation for Mouse Buttons.
pub type NativeMouseButton = sys::NSInteger;

/// Native OS Representation for Virtual Key Codes.
pub type NativeKeyCode = sys::c_ushort;

// -------------------------------------------------------------------------------------------------------------------------------- //

/// A button on a mouse.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct MouseButton(pub NativeMouseButton);

/// A key on a keyboard.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct KeyCode(pub NativeKeyCode);

// ================================================================================================================================ //
