/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Device-Surface Info
 */

//! Internal utilities for querying Device/Surface Information.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Information about a Device/Surface combination.
pub struct DeviceSurfaceInfo {
    /// Whether or not each Queue-Family supports the Surface.
    pub support: Vec<bool>,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html>
    pub modes: Vec<vk::PresentModeKHR>,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html>
    pub formats: Vec<vk::SurfaceFormatKHR>,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html>
    pub capabilities: vk::SurfaceCapabilitiesKHR,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl DeviceSurfaceInfo {
    /// Collects the information for the Device/Surface combination.
    pub fn new(device: &PhysicalDevice, surface: &Surface) -> Self {
        let surface_ext = unsafe { surface.surface_ext() };

        let support = (0..device.queues.len())
            .map(|idx| unsafe {
                let res = surface_ext.get_physical_device_surface_support(
                    device.handle,
                    idx as u32,
                    surface.handle,
                );
                res.unwrap()
            })
            .collect();

        let res = unsafe {
            surface_ext.get_physical_device_surface_present_modes(device.handle, surface.handle)
        };
        let modes = res.unwrap();

        let res = unsafe {
            surface_ext.get_physical_device_surface_formats(device.handle, surface.handle)
        };
        let formats = res.unwrap();

        let res = unsafe {
            surface_ext.get_physical_device_surface_capabilities(device.handle, surface.handle)
        };
        let capabilities = res.unwrap();

        Self {
            support,
            modes,
            formats,
            capabilities,
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl DeviceSurfaceInfo {
    /// Retrieves the ideal Surface Format.
    pub fn ideal_format(&self) -> vk::SurfaceFormatKHR {
        let find_format = |req_format: vk::Format, req_colorspace: vk::ColorSpaceKHR| {
            self.formats.iter().find(|format| {
                (format.format == req_format) && (format.color_space == req_colorspace)
            })
        };

        let srgb_bgra_u32 =
            find_format(vk::Format::B8G8R8A8_SRGB, vk::ColorSpaceKHR::SRGB_NONLINEAR);

        *srgb_bgra_u32.unwrap()
    }

    /// Retrieves the ideal Present Mode.
    #[allow(unused_variables)]
    pub fn ideal_mode(&self, vsync: bool) -> vk::PresentModeKHR {
        let find_mode =
            |req_mode: vk::PresentModeKHR| self.modes.iter().find(|mode| **mode == req_mode);

        // May tear, no vsync, reduced-latency.
        let immediate = find_mode(vk::PresentModeKHR::IMMEDIATE);

        // May tear, may vsync, reduced-stutter.
        let fifo_relaxed = find_mode(vk::PresentModeKHR::FIFO_RELAXED);

        // No tearing, must vsync, no frame-skips. (Required to be supported)
        let fifo = find_mode(vk::PresentModeKHR::FIFO);

        // No tearing, must vsync, may frame-skip.
        let mailbox = find_mode(vk::PresentModeKHR::MAILBOX);

        if vsync {
            *(fifo_relaxed).or(fifo).unwrap()
        } else {
            *(immediate).or(fifo_relaxed).or(fifo).unwrap()
        }
        //*(mailbox).or(fifo_relaxed).or(immediate).or(fifo).unwrap()
    }

    /// Retrieves the ideal Resolution.
    pub fn ideal_resolution(&self, surface: &Surface) -> vk::Extent2D {
        /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html#_description>
        const SPECIAL_VALUE: vk::Extent2D = vk::Extent2D {
            width: 0xFFFFFFFF,
            height: 0xFFFFFFFF,
        };

        let surface_size = surface.size;
        //let real_size = surface.real_size();

        let min_size = self.capabilities.min_image_extent;
        let max_size = self.capabilities.max_image_extent;
        let cur_size = self.capabilities.current_extent;

        if cur_size == SPECIAL_VALUE {
            surface_size
        } else {
            let width = (surface_size.width).clamp(min_size.width, max_size.width);
            let height = (surface_size.height).clamp(min_size.height, max_size.height);
            let size = vk::Extent2D { width, height };

            // if size != surface_size {
            //     eprintln!();
            //     eprintln!("        size: {size:?}");
            //     eprintln!("    cur_size: {cur_size:?}");
            //     eprintln!("    min_size: {min_size:?}");
            //     eprintln!("    max_size: {max_size:?}");
            //     eprintln!("surface_size: {surface_size:?}");
            //     eprintln!("   real_size: {real_size:?}");
            //     eprintln!();
            // }

            #[allow(clippy::let_and_return)]
            size
        }
    }

    /// Retrieves the ideal Viewport and Scissor.
    pub fn ideal_viewport_scissor(&self, surface: &Surface) -> (vk::Viewport, vk::Rect2D) {
        let image_res = self.ideal_resolution(surface);

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: image_res.width as f32,
            height: image_res.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: image_res,
        };

        (viewport, scissor)
    }
}

// ================================================================================================================================ //
