/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Buffers
 */

//! Internal utilities for creating Buffers in Device Memory to be used for Vertex/Index buffers.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// A group of buffers bound to Device Memory.
pub struct Buffers {
    /// Staging buffer.
    pub staging: Buffer,

    /// Index buffer.
    pub index: Buffer,

    /// Vertex buffer.
    pub vertex: Buffer,

    /// Device memory.
    pub memory: DeviceMemory,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
/// Size of a Kibibyte.
const KB: vk::DeviceSize = 0x400;

#[allow(unused)]
/// Size of a Mebibyte.
const MB: vk::DeviceSize = 0x400 * 0x400;

#[allow(unused)]
/// Size of a Gibibyte.
const GB: vk::DeviceSize = 0x400 * 0x400 * 0x400;

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Buffers {
    /// Creates the Buffers, allocates Device Memory, and binds the Buffers to the Device Memory.
    #[allow(clippy::identity_op)]
    pub fn new(device: &Device) -> Self {
        let vertex = Buffer::new(
            device,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::SharingMode::EXCLUSIVE,
            500 * MB,
        );

        let index = Buffer::new(
            device,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::SharingMode::EXCLUSIVE,
            1 * MB,
        );

        let staging = Buffer::new(
            device,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::SharingMode::EXCLUSIVE,
            500 * MB,
        );

        let alloc_size = vertex.mem_reqs.size + index.mem_reqs.size + staging.mem_reqs.size;

        let alloc_type = vertex.mem_reqs.memory_type_bits
            & index.mem_reqs.memory_type_bits
            & staging.mem_reqs.memory_type_bits;

        let alloc_props =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;

        let memory = DeviceMemory::new(device, alloc_size, alloc_type, alloc_props);

        let mut this = Self {
            staging,
            index,
            vertex,
            memory,
        };

        unsafe {
            this.vertex.bind(device, &this.memory, 0);
            this.index
                .bind(device, &this.memory, this.vertex.end_offs());
            this.staging
                .bind(device, &this.memory, this.index.end_offs());
        }

        this
    }
}

// ================================================================================================================================ //

/// Wrapper for a `VkBuffer`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuffer.html>
pub struct Buffer {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html>
    pub mem_reqs: vk::MemoryRequirements,

    /// Inner `VkBuffer`.
    pub handle: vk::Buffer,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html>
    pub sharing_mode: vk::SharingMode,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html>
    pub usage: vk::BufferUsageFlags,

    /// Byte-Size of this Buffer.
    pub size: vk::DeviceSize,

    /// Byte-Offset of this Buffer.
    pub offs: vk::DeviceSize,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Buffer {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Buffer {
    /// Creates a new Buffer of the given size.
    pub fn new(
        device: &Device,
        usage: vk::BufferUsageFlags,
        sharing_mode: vk::SharingMode,
        size: vk::DeviceSize,
    ) -> Self {
        let device_ptr = NonNull::from(device);

        let create_info = vk::BufferCreateInfo {
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage,
            sharing_mode,
            ..Default::default()
        };

        let res = unsafe { device.ash_device().create_buffer(&create_info, None) };
        let handle = res.unwrap();

        let mem_reqs = unsafe { device.ash_device().get_buffer_memory_requirements(handle) };

        Self {
            mem_reqs,
            handle,
            sharing_mode,
            usage,
            size,
            offs: 0,
            device_ptr,
        }
    }

    /// Binds the Buffer to the Device Memory at the given offset.
    pub unsafe fn bind(&mut self, device: &Device, memory: &DeviceMemory, offset: vk::DeviceSize) {
        let res = device
            .ash_device()
            .bind_buffer_memory(self.handle, memory.handle, offset);

        res.unwrap();

        self.offs = offset;
    }

    /// Offset of the End of this Buffer.
    pub fn end_offs(&self) -> vk::DeviceSize {
        self.offs + self.size
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_buffer(self.handle, None) };
    }
}

// ================================================================================================================================ //
