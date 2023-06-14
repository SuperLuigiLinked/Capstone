/*
 *  Crate: RGE
 * Module: Vulkan - Utils - Extensions
 */

//! Internal utilities for enumerating Required and Supported Vulkan Extensions for Instances and Devices.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// A list of Supported Extensions.
pub struct Extensions {
    /// The Properties of each Extension.
    pub list: Vec<vk::ExtensionProperties>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Extensions {
    /// Collects the Supported Extensions for Vulkan Instances.
    pub fn supported_instance(entry: &ash::Entry) -> Self {
        let list = entry.enumerate_instance_extension_properties(None).unwrap();
        Self { list }
    }

    /// Collects the Supported Extensions for a given Vulkan Instance + Physical Device.
    pub fn supported_device(instance: &ash::Instance, device: vk::PhysicalDevice) -> Self {
        let res = unsafe { instance.enumerate_device_extension_properties(device) };

        let list = res.unwrap();

        Self { list }
    }

    /// Collects the Names of Required Instance Extensions.
    pub fn required_instance() -> Vec<NtString<'static>> {
        vec![
            SURFACE,
            #[cfg(target_os = "windows")]
            WIN32_SURFACE,
            #[cfg(target_os = "linux")]
            XCB_SURFACE,
            #[cfg(target_os = "macos")]
            METAL_SURFACE,
            #[cfg(debug_assertions)]
            DEBUG_UTILS,
        ]
    }

    /// Collects the Names of Required Device Extensions.
    pub fn required_device() -> Vec<NtString<'static>> {
        vec![SWAPCHAIN]
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Extensions {
    /// Returns an Iterator over the Names of each Supported Extension.
    pub fn names(&self) -> impl Iterator<Item = NtString> {
        self.list.iter().map(|extension| {
            let name = &extension.extension_name;
            NtString::from_chars_cap(name, name.len())
        })
    }

    /// Returns whether or not the Extension provided by `name` is supported.
    pub fn supports(&self, name: &NtString) -> bool {
        self.names().any(|sup_name| sup_name == *name)
    }

    /// Returns whether or not the Extensions provided by `names` are supported.
    pub fn supports_all(&self, names: &[NtString]) -> bool {
        names.iter().all(|name| self.supports(name))
    }

    #[allow(unused)]
    /// Returns an Iterator over the Supported Names.
    pub fn supported<'a>(
        &'a self,
        names: impl Iterator<Item = &'a NtString<'a>>,
    ) -> impl Iterator<Item = &'a NtString<'a>> {
        names.filter(|name| self.supports(name))
    }

    /// Returns an Iterator over the Unsupported Names.
    pub fn unsupported<'a>(
        &'a self,
        names: impl Iterator<Item = &'a NtString<'a>>,
    ) -> impl Iterator<Item = &'a NtString<'a>> {
        names.filter(|name| !self.supports(name))
    }
}

// ================================================================================================================================ //

use names::*;

/// Constants for Extension Names.
#[allow(unused)]
mod names {
    use super::*;

    // ---------------------------------------------------------------- //

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html>
    pub const SURFACE: NtString = NtString::from_bytes(b"VK_KHR_surface\0");

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html>
    pub const WIN32_SURFACE: NtString = NtString::from_bytes(b"VK_KHR_win32_surface\0");

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_xcb_surface.html>
    pub const XCB_SURFACE: NtString = NtString::from_bytes(b"VK_KHR_xcb_surface\0");

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_surface.html>
    pub const METAL_SURFACE: NtString = NtString::from_bytes(b"VK_EXT_metal_surface\0");

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html>
    pub const DEBUG_UTILS: NtString = NtString::from_bytes(b"VK_EXT_debug_utils\0");

    // ---------------------------------------------------------------- //

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html>
    pub const SWAPCHAIN: NtString = NtString::from_bytes(b"VK_KHR_swapchain\0");

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //
