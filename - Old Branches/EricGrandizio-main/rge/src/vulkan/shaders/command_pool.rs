/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Command Pool
 */

//! Internal utilities for creating Command Pools to allocate Command Buffers.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkCommandPool`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html>
pub struct CommandPool {
    /// Inner `VkCommandPool`.
    pub handle: vk::CommandPool,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl CommandPool {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl CommandPool {
    /// Creates a new Command Pool.
    pub fn new(device: &Device) -> Self {
        let device_ptr = NonNull::from(device);

        let create_info = vk::CommandPoolCreateInfo {
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            queue_family_index: device.selection.graphics_idx as u32,
            ..Default::default()
        };
        let res = unsafe { device.ash_device().create_command_pool(&create_info, None) };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_command_pool(self.handle, None) };
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl CommandPool {
    /// Allocates `count` Command Buffers from this pool.
    pub fn allocate_buffers(&self, count: usize) -> Vec<vk::CommandBuffer> {
        let alloc_info = vk::CommandBufferAllocateInfo {
            level: vk::CommandBufferLevel::PRIMARY,
            command_pool: self.handle,
            command_buffer_count: count as u32,
            ..Default::default()
        };

        let res = unsafe { self.ash_device().allocate_command_buffers(&alloc_info) };
        res.unwrap()
    }
}

// ================================================================================================================================ //
