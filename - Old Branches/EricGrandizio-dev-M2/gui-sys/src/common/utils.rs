/*
 *  Crate: GUI-Sys
 * Module: Common - Utils
 */

//! Common utils.

// ================================================================================================================================ //

pub use super::c_types::*;
pub use ::core::mem::{size_of, transmute, transmute_copy, zeroed, MaybeUninit};
pub use ::core::ptr::{addr_of, addr_of_mut, null, null_mut};
pub use ::std::ffi::{CStr, CString, OsStr, OsString};

#[cfg(target_os = "windows")]
pub use ::std::os::windows::prelude::{OsStrExt, OsStringExt};

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Gets a `*const void` to the given data.
#[macro_export]
macro_rules! void_of {
    ($expr:expr) => {
        ::core::ptr::addr_of!($expr) as *const ::core::ffi::c_void
    };
}

/// Gets a `*mut void` to the given data.
#[macro_export]
macro_rules! void_of_mut {
    ($expr:expr) => {
        ::core::ptr::addr_of_mut!($expr) as *mut ::core::ffi::c_void
    };
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Gets a `*const char` to the given data.
#[macro_export]
macro_rules! bytes_of {
    ($expr:expr) => {
        ::core::ptr::addr_of!($expr) as *const ::core::ffi::c_char
    };
}

/// Gets a `*mut char` to the given data.
#[macro_export]
macro_rules! bytes_of_mut {
    ($expr:expr) => {
        ::core::ptr::addr_of_mut!($expr) as *mut ::core::ffi::c_char
    };
}

// ================================================================================================================================ //
