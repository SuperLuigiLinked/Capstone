/*
 *  Crate: RGE
 * Module: Vulkan - Utils
 */

//! Internal utilities for Miscellaneous purposes.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

pub(super) use core::ptr::NonNull;
pub(super) use std::borrow::Cow;

pub(super) use ash::extensions::ext;
pub(super) use ash::extensions::khr;
pub(super) use ash::vk;

pub(super) use wyn::window::WindowHandle;

// ================================================================================================================================ //

mod aliased;
pub use aliased::*;

mod nt_string;
pub use nt_string::*;

mod layers;
pub use layers::*;

mod extensions;
pub use extensions::*;

// ================================================================================================================================ //

/// Converts a Slice of any type to a Slice over its bytes.
#[inline]
pub const fn bytes_ref<T>(slice: &[T]) -> &[u8] {
    let len = core::mem::size_of::<T>() * slice.len();
    let data = slice.as_ptr() as *const u8;
    unsafe { core::slice::from_raw_parts(data, len) }
}

// ================================================================================================================================ //

/// Converts a Vulkan API Version number to Text.
#[allow(unused)]
pub fn api_version(version: u32) -> String {
    let variant = vk::api_version_variant(version);
    let major = vk::api_version_major(version);
    let minor = vk::api_version_minor(version);
    let patch = vk::api_version_patch(version);
    format!("{variant}.{major}.{minor}.{patch}")
}

// ================================================================================================================================ //
