/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Sync
 */

//! Internal utilities for managing CPU <-> GPU and GPU <-> GPU Synchronization.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkSemaphore`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html>
pub struct Semaphore {
    /// Inner `VkSemaphore`.
    pub handle: vk::Semaphore,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Semaphore {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Semaphore {
    /// Creates a new Vulkan Semaphore.
    pub fn new(device: &Device) -> Self {
        let device_ptr = NonNull::from(device);

        let create_info = vk::SemaphoreCreateInfo {
            ..Default::default()
        };

        let res = unsafe { device.ash_device().create_semaphore(&create_info, None) };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_semaphore(self.handle, None) };
    }
}

// ================================================================================================================================ //

/// Wrapper for a `VkFence`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFence.html>
pub struct Fence {
    /// Inner `VkFence`.
    pub handle: vk::Fence,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Fence {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Fence {
    /// Creates a new Vulkan Fence, using the provided signaled state.
    pub fn new(device: &Device, signaled: bool) -> Self {
        let device_ptr = NonNull::from(device);

        let signal_flag = if signaled {
            vk::FenceCreateFlags::SIGNALED
        } else {
            vk::FenceCreateFlags::default()
        };

        let create_info = vk::FenceCreateInfo {
            flags: signal_flag,
            ..Default::default()
        };

        let res = unsafe { device.ash_device().create_fence(&create_info, None) };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_fence(self.handle, None) };
    }
}

// ================================================================================================================================ //

/// A collection of Synchronization Objects used by a `Frame` during Rendering.
pub struct Sync {
    /// Semaphore to wait until an Image can be Acquired.
    pub image: Semaphore,

    /// Semaphore to signal when Rendering has Finished.
    pub render: Semaphore,

    /// Fence to block CPU access while a Frame is In-Use.
    pub frame: Fence,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Sync {
    /// Creates new Synchronization primitives for the given Device.
    pub fn new(device: &Device) -> Self {
        let image = Semaphore::new(device);
        let render = Semaphore::new(device);
        let frame = Fence::new(device, true);

        Self {
            image,
            render,
            frame,
        }
    }
}

// ================================================================================================================================ //
