/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Memory
 */

//! Internal utilities for allocating and mapping Memory on Vulkan Devices (such as GPUs).

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkDeviceMemory`.
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html>
pub struct DeviceMemory {
    /// Inner `VkDeviceMemory`.
    pub handle: vk::DeviceMemory,

    /// Size of the allocated memory, in bytes.
    pub capacity: vk::DeviceSize,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl DeviceMemory {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl DeviceMemory {
    /// Allocates Device Memory suitable for the given requirements.
    pub fn new(
        device: &Device,
        capacity: vk::DeviceSize,
        memory_bits: u32,
        memory_props: vk::MemoryPropertyFlags,
    ) -> Self {
        let device_ptr = NonNull::from(device);

        let physical = unsafe { device.selection.physical() };

        let res = physical.find_memory_type(memory_bits, memory_props);
        let (mem_idx, _mem_type) = res.unwrap();

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: capacity,
            memory_type_index: mem_idx as u32,
            ..Default::default()
        };

        let res = unsafe { device.ash_device().allocate_memory(&alloc_info, None) };
        let handle = res.unwrap();

        Self {
            handle,
            capacity,
            device_ptr,
        }
    }

    /// Maps the Device Memory into a CPU-visible region.
    pub unsafe fn map(&self, offset: vk::DeviceSize, size: vk::DeviceSize) -> MappedMemory {
        assert!(
            size <= self.capacity,
            "{} bytes of GPU Vertex Memory are available, but {} bytes were requested",
            self.capacity,
            size
        );

        MappedMemory::new(self.device(), self, offset, size.max(1))
    }
}

impl Drop for DeviceMemory {
    fn drop(&mut self) {
        unsafe { self.ash_device().free_memory(self.handle, None) };
    }
}

// ================================================================================================================================ //

/// Device Memory that was mapped to a CPU-visible region.
pub struct MappedMemory<'a> {
    /// Number of mapped bytes.
    size: vk::DeviceSize,

    /// Pointer to mapped bytes.
    buffer: *mut c_void,

    /// Memory this was mapped from.
    memory: &'a DeviceMemory,

    /// Device that holds the memory.
    device: &'a Device,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl<'a> MappedMemory<'a> {
    /// Maps `size` bytes at `offset` bytes into the provided Device Memory.
    pub fn new(
        device: &'a Device,
        memory: &'a DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> Self {
        let res = unsafe {
            device
                .ash_device()
                .map_memory(memory.handle, offset, size, vk::MemoryMapFlags::empty())
        };
        let buffer = res.unwrap();

        Self {
            size,
            buffer,
            memory,
            device,
        }
    }
}

impl<'a> Drop for MappedMemory<'a> {
    fn drop(&mut self) {
        unsafe { self.device.ash_device().unmap_memory(self.memory.handle) };
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl<'a> MappedMemory<'a> {
    /// Returns a reference to the mapped bytes.
    #[allow(dead_code)]
    pub fn bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buffer as *const u8, self.size as usize) }
    }

    /// Returns a mutable reference to the mapped bytes.
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.buffer as *mut u8, self.size as usize) }
    }
}

// ================================================================================================================================ //
