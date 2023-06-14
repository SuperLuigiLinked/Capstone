/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Pipeline Layout
 */

//! Internal utilities for defining Vulkan Pipeline Layouts.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkPipelineLayout`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html>
pub struct PipelineLayout {
    /// Inner `VkPipelineLayout`.
    pub handle: vk::PipelineLayout,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl PipelineLayout {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl PipelineLayout {
    /// Creates the Pipeline Layout.
    pub fn new(device: &Device, create_info: &vk::PipelineLayoutCreateInfo) -> Self {
        let device_ptr = NonNull::from(device);

        let res = unsafe {
            device
                .ash_device()
                .create_pipeline_layout(create_info, None)
        };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_pipeline_layout(self.handle, None) };
    }
}

// ================================================================================================================================ //
