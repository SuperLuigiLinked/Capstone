/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Render Pass
 */

//! Internal utilities for defining Vulkan Render Passes.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkRenderPass`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html>
pub struct RenderPass {
    /// Inner `VkRenderPass`.
    pub handle: vk::RenderPass,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl RenderPass {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl RenderPass {
    /// Creates a new Render Pass.
    pub fn new(device: &Device, create_info: &vk::RenderPassCreateInfo) -> Self {
        let device_ptr = NonNull::from(device);

        let res = unsafe { device.ash_device().create_render_pass(create_info, None) };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_render_pass(self.handle, None) };
    }
}

// ================================================================================================================================ //
