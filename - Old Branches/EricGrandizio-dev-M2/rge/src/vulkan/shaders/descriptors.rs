/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Descriptor Set Layout
 */

//! Internal utilities for defining Vulkan Descriptor Set Layouts.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkDescriptorSetLayout`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html>
pub struct DescriptorSetLayout {
    /// Inner `VkDescriptorSetLayout`.
    pub handle: vk::DescriptorSetLayout,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl DescriptorSetLayout {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl DescriptorSetLayout {
    /// Creates the Descriptor Set Layout.
    pub fn new(device: &Device, create_info: &vk::DescriptorSetLayoutCreateInfo) -> Self {
        let device_ptr = NonNull::from(device);

        let res = unsafe {
            device
                .ash_device()
                .create_descriptor_set_layout(create_info, None)
        };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self.ash_device()
                .destroy_descriptor_set_layout(self.handle, None)
        };
    }
}

// ================================================================================================================================ //

/// Wrapper for a `VkDescriptorPool`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPool.html>
pub struct DescriptorPool {
    /// Inner `VkDescriptorSetLayout`.
    pub handle: vk::DescriptorPool,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl DescriptorPool {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl DescriptorPool {
    #[allow(unused)]
    /// Creates the Descriptor Set Layout.
    pub fn new(device: &Device, create_info: &vk::DescriptorPoolCreateInfo) -> Self {
        let device_ptr = NonNull::from(device);

        let res = unsafe {
            device
                .ash_device()
                .create_descriptor_pool(create_info, None)
        };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_descriptor_pool(self.handle, None) };
    }
}

// ================================================================================================================================ //
