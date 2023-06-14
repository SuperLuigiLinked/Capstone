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

mod arrays;
pub use arrays::*;

mod layers;
pub use layers::*;

mod extensions;
pub use extensions::*;

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

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Finds the Index of the Memory Type that meets the given requirements.
pub fn find_memory_type(
    types: &[vk::MemoryType],
    memory_bits: u32,
    memory_props: vk::MemoryPropertyFlags,
) -> Option<(usize, &vk::MemoryType)> {
    types.iter().enumerate().find(|(idx, mem_type)| {
        let this_bit = 1 << idx;

        let has_bit = (this_bit & memory_bits) != 0;
        let has_props = (mem_type.property_flags & memory_props) == memory_props;

        has_bit && has_props
    })
}

// ================================================================================================================================ //

/// Unit-Test for API Version Strings.
#[test]
fn api_version_test() {
    let variant = 1u32;
    let major = 2u32;
    let minor = 3u32;
    let patch = 4u32;

    // From Vulkan Spec: <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_MAKE_API_VERSION.html>
    let version = (variant << 29) | (major << 22) | (minor << 12) | patch;

    let expected = "1.2.3.4";
    let result = self::api_version(version);

    assert_eq!(result, expected)
}

// ================================================================================================================================ //
