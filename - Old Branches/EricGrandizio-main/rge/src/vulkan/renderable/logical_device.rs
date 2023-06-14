/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Logical Device
 */

//! Internal utilities for managing Vulkan Logical Devices and Queues.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkDevice`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevice.html>
pub struct LogicalDevice {
    /// Queue for Presenting Images to a Surface.
    pub present_queue: vk::Queue,

    /// Queue for Submitting Graphics commands.
    pub graphics_queue: vk::Queue,

    /// Inner `ash` `VkDevice`.
    pub inner: ash::Device,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl LogicalDevice {
    /// Creates a Logical Device from the given 
    pub fn new(instance: &ash::Instance, selection: &DeviceSelection) -> Self {
        let graphics_queue_create_info = vk::DeviceQueueCreateInfo {
            queue_family_index: selection.graphics_idx as u32,
            queue_count: 1,
            p_queue_priorities: &1.0_f32,
            ..Default::default()
        };
        let present_queue_create_info = vk::DeviceQueueCreateInfo {
            queue_family_index: selection.present_idx as u32,
            queue_count: 1,
            p_queue_priorities: &1.0_f32,
            ..Default::default()
        };

        let queue_create_infos = [graphics_queue_create_info, present_queue_create_info];

        let device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let required_extensions = Extensions::required_device();
        let required_layers = Layers::required_device();

        let device_create_info = vk::DeviceCreateInfo {
            p_enabled_features: &device_features,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            queue_create_info_count: selection.unique_queue_count(),
            pp_enabled_extension_names: required_extensions.as_ptr() as *const *const c_char,
            enabled_extension_count: required_extensions.len() as u32,
            pp_enabled_layer_names: required_layers.as_ptr() as *const *const c_char,
            enabled_layer_count: required_layers.len() as u32,
            ..Default::default()
        };

        let res = unsafe {
            instance.create_device(selection.physical().handle, &device_create_info, None)
        };
        let inner = res.unwrap();

        let graphics_queue = unsafe { inner.get_device_queue(selection.graphics_idx as u32, 0) };
        let present_queue = unsafe { inner.get_device_queue(selection.present_idx as u32, 0) };

        Self {
            present_queue,
            graphics_queue,
            inner,
        }
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe { self.inner.destroy_device(None) };
    }
}

// ================================================================================================================================ //
