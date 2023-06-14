/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Device Exts
 */

//! Internal utilities for loading and using Device Extension functions.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Collection of loaded Device Extension functions.
pub struct DeviceExts {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html>
    pub swapchain: khr::Swapchain,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl DeviceExts {
    /// Loads and stores the various Device Extensions.
    pub fn new(instance: &ash::Instance, device: &ash::Device) -> Aliased<Self> {
        let swapchain = khr::Swapchain::new(instance, device);

        let this = Self { swapchain };
        Aliased::new(this)
    }
}

// ================================================================================================================================ //
