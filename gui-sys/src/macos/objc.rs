/*
 *  Crate: GUI-Sys
 * Module: MacOS - ObjC
 */

//! MacOS Objective-C bindings.
//!
//! # Dependencies
//! * <https://docs.rs/objc/latest/objc/>
//!
//! # Documentation
//! * <https://developer.apple.com/documentation/technologies>

// -------------------------------------------------------------------------------------------------------------------------------- //

use crate::common::c_types::*;

// ================================================================================================================================ //
// Macros
// -------------------------------------------------------------------------------------------------------------------------------- //

/// Declares an Objective-C Class represented by an `id`.
#[macro_export]
macro_rules! id_class {
    ($name:ident) => {
        pub type $name = ::objc::runtime::Object;
    };
}

/// Declares an Objective-C Protocol represented by an `id`.
#[macro_export]
macro_rules! id_protocol {
    ($name:ident) => {
        pub type $name = ::objc::runtime::Object;
    };
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/objectivec/1456712-objc_msgsend>\
pub use ::objc::msg_send;

/// <https://developer.apple.com/documentation/objectivec/1418952-objc_getclass?language=objc>\
pub use ::objc::class;

/// <https://developer.apple.com/documentation/objectivec/1418557-sel_registername?language=objc>\
pub use ::objc::sel;
pub use ::objc::sel_impl;

// ================================================================================================================================ //
// Types
// -------------------------------------------------------------------------------------------------------------------------------- //

/// ...
pub use ::objc::{
    declare::{ClassDecl, ProtocolDecl},
    runtime::{Class, Imp, Ivar, Method, Object, Protocol, Sel},
};

/// <https://developer.apple.com/documentation/objectivec/id?language=objc>
pub type id = *mut Object;

/// <https://developer.apple.com/documentation/objectivec/bool?language=objc>
pub use ::objc::runtime::BOOL;

// ================================================================================================================================ //
// Functions
// -------------------------------------------------------------------------------------------------------------------------------- //

// ================================================================================================================================ //
// Constants
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/objectivec/no>
pub use ::objc::runtime::NO;
/// <https://developer.apple.com/documentation/objectivec/yes>
pub use ::objc::runtime::YES;

/// <https://developer.apple.com/documentation/objectivec/nil-2gl/>
pub const nil: id = ::core::ptr::null_mut();

// ================================================================================================================================ //
