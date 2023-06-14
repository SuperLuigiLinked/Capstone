/*
 *  Crate: RGE
 * Module: Vulkan - Context - Instance Exts
 */

//! Internal utilities for loading and using Instance Extension functions.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

#[cfg(target_os = "windows")]
/// OS-specific Surface Extension.
pub type NativeSurfaceExt = khr::Win32Surface;

#[cfg(target_os = "linux")]
/// OS-specific Surface Extension.
pub type NativeSurfaceExt = khr::XcbSurface;

#[cfg(target_os = "macos")]
/// OS-specific Surface Extension.
pub type NativeSurfaceExt = ext::MetalSurface;

// ================================================================================================================================ //

/// Collection of loaded Instance Extension functions.
pub struct InstanceExts {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html>
    pub surface: khr::Surface,

    /// Windows: <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html>\
    ///   Linux: <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_xcb_surface.html>\
    ///   MacOS: <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_surface.html>
    pub native_surface: NativeSurfaceExt,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html>
    pub debug_utils: ext::DebugUtils,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl InstanceExts {
    /// Loads and stores the various Instance Extensions.
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Aliased<Self> {
        let surface = khr::Surface::new(entry, instance);

        let native_surface = NativeSurfaceExt::new(entry, instance);

        let debug_utils = ext::DebugUtils::new(entry, instance);

        let this = Self {
            surface,
            native_surface,
            debug_utils,
        };
        Aliased::new(this)
    }
}

// ================================================================================================================================ //
