/*
 *  Crate: RGE
 * Module: Vulkan - Context - Physical Device
 */

//! Internal utilities for querying and managing Vulkan Physical Devices.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkPhysicalDevice` and associated state.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html>
pub struct PhysicalDevice {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html>
    pub queues: Vec<vk::QueueFamilyProperties>,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html>
    pub memory: vk::PhysicalDeviceMemoryProperties,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html>
    pub features: vk::PhysicalDeviceFeatures,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html>
    pub properties: vk::PhysicalDeviceProperties,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLayerProperties.html>
    pub layers: Layers,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtensionProperties.html>
    pub extensions: Extensions,

    /// Inner `VkPhysicalDevice`.
    pub handle: vk::PhysicalDevice,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl PhysicalDevice {
    /// Queries all available Physical Devices.
    pub fn collect(instance: &ash::Instance) -> Vec<Self> {
        let res = unsafe { instance.enumerate_physical_devices() };
        let handles = res.unwrap();

        handles
            .into_iter()
            .map(|handle| Self::new(instance, handle))
            .collect()
    }

    /// Wraps a handle to a Physical Device with its associated state.
    pub fn new(instance: &ash::Instance, handle: vk::PhysicalDevice) -> Self {
        let extensions = Extensions::supported_device(instance, handle);
        let layers = Layers::supported_device(instance, handle);

        let properties = unsafe { instance.get_physical_device_properties(handle) };
        let features = unsafe { instance.get_physical_device_features(handle) };
        let memory = unsafe { instance.get_physical_device_memory_properties(handle) };

        let queues = unsafe { instance.get_physical_device_queue_family_properties(handle) };

        Self {
            queues,
            memory,
            features,
            properties,
            layers,
            extensions,
            handle,
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl PhysicalDevice {
    /// Returns a slice over the Memory Types supported by this Physical Device.
    pub fn memory_types(&self) -> &[vk::MemoryType] {
        let len = self.memory.memory_type_count as usize;
        &self.memory.memory_types[0..len]
    }

    /// Returns a slice over the Memory Heaps supported by this Physical Device.
    #[allow(dead_code)]
    pub fn memory_heaps(&self) -> &[vk::MemoryHeap] {
        let len = self.memory.memory_heap_count as usize;
        &self.memory.memory_heaps[0..len]
    }
}

// ================================================================================================================================ //
