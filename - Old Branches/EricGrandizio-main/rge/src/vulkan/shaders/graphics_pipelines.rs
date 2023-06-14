/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Graphics Pipelines
 */

//! Internal utilities for creating Vulkan Graphics Pipelines.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for `VkPipeline`s used for Graphics operations.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipeline.html>
pub struct GraphicsPipelines {
    /// Inner `VkPipeline`s.
    pub handles: Vec<vk::Pipeline>,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl GraphicsPipelines {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl GraphicsPipelines {
    /// Creates new Graphics Pipelines.
    pub fn new(device: &Device, create_infos: &[vk::GraphicsPipelineCreateInfo]) -> Self {
        let device_ptr = NonNull::from(device);

        let res = unsafe {
            device.ash_device().create_graphics_pipelines(
                vk::PipelineCache::null(),
                create_infos,
                None,
            )
        };
        let handles = res.unwrap();

        Self {
            handles,
            device_ptr,
        }
    }
}

impl Drop for GraphicsPipelines {
    fn drop(&mut self) {
        for handle in self.handles.iter().cloned() {
            unsafe { self.ash_device().destroy_pipeline(handle, None) };
        }
    }
}

// ================================================================================================================================ //
