/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Frames
 */

//! Internal utilities for managing Vulkan Framebuffers.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Handles to objects associated with a Frame that can be rendered to and presented to the screen.
pub struct Frame {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html>
    pub commands: vk::CommandBuffer,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html>
    pub buffer: vk::Framebuffer,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageView.html>
    pub view: vk::ImageView,

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImage.html>
    pub image: vk::Image,

    /// Synchronization Primitives for accessing this Frame.
    pub sync: Sync,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Frame {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Frame {
    /// Creates a new Frame with the provided `VkImage` and `VkCommandBuffer`.
    pub fn new(
        device: &Device,
        surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        render_pass: &RenderPass,
        image: vk::Image,
        commands: vk::CommandBuffer,
    ) -> Self {
        let device_ptr = NonNull::from(device);

        let sync = Sync::new(device);

        let view = vk::ImageView::null();
        let buffer = vk::Framebuffer::null();

        let mut this = Self {
            sync,
            buffer,
            view,
            image,
            commands,
            device_ptr,
        };

        this.view = this.create_view(device, ds_info);
        this.buffer = this.create_buffer(device, surface, ds_info, render_pass);

        this
    }

    /// Creates a `VkImageView` associated with this Frame.
    pub fn create_view(&self, device: &Device, ds_info: &DeviceSurfaceInfo) -> vk::ImageView {
        let image_format = ds_info.ideal_format().format;

        let create_info = vk::ImageViewCreateInfo {
            image: self.image,
            format: image_format,
            view_type: vk::ImageViewType::TYPE_2D,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: 1,
                layer_count: 1,
                ..Default::default()
            },

            ..Default::default()
        };
        let res = unsafe { device.ash_device().create_image_view(&create_info, None) };
        res.unwrap()
    }

    /// Creates a `VkFramebuffer` associated with this Frame.
    pub fn create_buffer(
        &self,
        device: &Device,
        surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        render_pass: &RenderPass,
    ) -> vk::Framebuffer {
        let image_res = ds_info.ideal_resolution(surface);

        let create_info = vk::FramebufferCreateInfo {
            render_pass: render_pass.handle,
            attachment_count: 1,
            p_attachments: addr_of!(self.view),
            width: image_res.width,
            height: image_res.height,
            layers: 1,
            ..Default::default()
        };
        let res = unsafe { device.ash_device().create_framebuffer(&create_info, None) };
        res.unwrap()
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            let ash_device = self.ash_device();

            if self.buffer != vk::Framebuffer::null() {
                ash_device.destroy_framebuffer(self.buffer, None);
            }

            if self.view != vk::ImageView::null() {
                ash_device.destroy_image_view(self.view, None);
            }
        }
    }
}

// ================================================================================================================================ //

/// A collection of Multiple Frames owned by a Swapchain.
pub struct Frames {
    /// The list of Frames.
    pub list: Vec<Frame>,

    /// The Swapchain that owns the Frames.
    pub swapchain: Swapchain,

    /// The `VkCommandBuffers` allocated from the `VkCommandPool`.
    pub commands: Vec<vk::CommandBuffer>,

    /// The Command Pool for allocating Command Buffers.
    pub pool: CommandPool,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Frames {
    /// Creates a Swapchain for the given Device/Surface and collects the Frames associated with it.
    pub fn collect(
        device: &Device,
        surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        render_pass: &RenderPass,
        vsync: bool,
    ) -> Self {
        let pool = CommandPool::new(device);
        let swapchain = Swapchain::new(device, surface, ds_info, vsync, None);

        let images = swapchain.collect_images();
        let commands = pool.allocate_buffers(images.len());
        assert_eq!(images.len(), commands.len());
        let zipped = (images.into_iter()).zip(commands.iter().cloned());

        let list = zipped
            .map(|(image, commands)| {
                Frame::new(device, surface, ds_info, render_pass, image, commands)
            })
            .collect();

        Self {
            list,
            swapchain,

            commands,
            pool,
        }
    }

    /// Re-creates the Swapchain.
    pub fn update(
        &mut self,
        surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        render_pass: &RenderPass,
        vsync: bool,
    ) {
        let device = unsafe { self.pool.device() };
        unsafe { device.ash_device().device_wait_idle().unwrap() };

        self.list.clear();

        let old_swapchain = Some(self.swapchain.handle);
        self.swapchain = Swapchain::new(device, surface, ds_info, vsync, old_swapchain);

        let images = self.swapchain.collect_images();
        // let commands = pool.allocate_buffers(images.len());
        assert_eq!(images.len(), self.commands.len());
        let zipped = (images.into_iter()).zip(self.commands.iter().cloned());

        self.list = zipped
            .map(|(image, commands)| {
                Frame::new(device, surface, ds_info, render_pass, image, commands)
            })
            .collect();
    }
}

// ================================================================================================================================ //
