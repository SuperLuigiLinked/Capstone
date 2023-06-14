/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Swapchain
 */

//! Internal utilities for managing Vulkan Swapchains.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkSwapchainKHR`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html>
pub struct Swapchain {
    /// Whether or not the Swapchain is currently prioritizing VSYNC-enabled Presentation Modes.
    pub vsync: bool,

    /// Inner `VkSwapchainKHR`.
    pub handle: vk::SwapchainKHR,

    /// Pointer to the object responsible for freeing this resource.
    ext_ptr: NonNull<khr::Swapchain>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Swapchain {
    pub unsafe fn ext(&self) -> &khr::Swapchain {
        self.ext_ptr.as_ref()
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Swapchain {
    /// Creates a new Swapchain for the Device/Surface combination.\
    /// If the Swapchain is being re-created, the old handle can be passed in.
    pub fn new(
        device: &Device,
        surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        vsync: bool,
        old_swapchain: Option<vk::SwapchainKHR>,
    ) -> Self {
        let exts_ref = unsafe { device.exts() };
        let ext_ptr = NonNull::from(&exts_ref.swapchain);

        let min_images = ds_info.capabilities.min_image_count;
        let max_images = ds_info.capabilities.max_image_count;
        let req_images = min_images + 1;

        // <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html#_description>
        let num_images = if max_images == 0 {
            req_images
        } else {
            req_images.min(max_images)
        };

        let best_format = ds_info.ideal_format();
        let best_present = ds_info.ideal_mode(vsync);
        let best_res = ds_info.ideal_resolution(surface);

        let queue_indices = device.selection.queue_indices();
        let queue_count = device.selection.concurrent_queue_count();
        let queue_mode = device.selection.queue_mode();
        let queue_ptr = if device.selection.queues_unique() {
            queue_indices.as_ptr()
        } else {
            null()
        };

        let create_info = vk::SwapchainCreateInfoKHR {
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: surface.handle,
            min_image_count: num_images,
            image_format: best_format.format,
            image_color_space: best_format.color_space,
            present_mode: best_present,
            image_extent: best_res,
            image_array_layers: 1,
            p_queue_family_indices: queue_ptr,
            queue_family_index_count: queue_count,
            image_sharing_mode: queue_mode,
            pre_transform: ds_info.capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            clipped: vk::TRUE,
            old_swapchain: old_swapchain.unwrap_or_default(),
            ..Default::default()
        };

        let res = unsafe { exts_ref.swapchain.create_swapchain(&create_info, None) };
        let handle = res.unwrap();

        Self {
            vsync,
            handle,
            ext_ptr,
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe { self.ext().destroy_swapchain(self.handle, None) };
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Swapchain {
    /// Collects all the Images associated with this Swapchain.
    pub fn collect_images(&self) -> Vec<vk::Image> {
        let res = unsafe { self.ext().get_swapchain_images(self.handle) };
        res.unwrap()
    }
}

// ================================================================================================================================ //
