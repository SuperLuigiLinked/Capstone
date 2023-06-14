/*
 *  Crate: RGE
 * Module: Vulkan - Renderable
 */

//! Internal utilities for managing Renderable-State associated with a Window in Vulkan.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

mod surface;
pub use surface::*;

mod ds_info;
pub use ds_info::*;

mod device_selection;
pub use device_selection::*;

mod logical_device;
pub use logical_device::*;

mod device;
pub use device::*;

mod device_exts;
pub use device_exts::*;

mod swapchain;
pub use swapchain::*;

mod sync;
pub use sync::*;

mod frames;
pub use frames::*;

// ================================================================================================================================ //

/// A combination of a Vulkan Device/Surface with its associated Shaders & Graphics Pipelines.
pub struct VulkanRenderable {
    /// Index of the current Frame.
    pub frame_idx: usize,

    /// List of Frames to be rendered.
    pub frames: Frames,

    /// Compiled Shader Programs.
    pub shaders: Shaders,

    /// Texture atlases.
    pub atlas: Option<Atlas>,

    /// The Command Pool for allocating Command Buffers.
    pub command_pool: CommandPool,

    /// Memory Buffers for CPU-GPU interaction.
    pub buffers: Buffers,

    /// Information about the Device/Surface combination.
    pub ds_info: DeviceSurfaceInfo,

    /// The Logical Device for using Vulkan.
    pub device: Aliased<Device>,

    /// The Window Surface.
    pub surface: Surface,

    /// Pointer to the object responsible for freeing this resource.
    context_ptr: NonNull<VulkanContext>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl VulkanRenderable {
    pub unsafe fn context(&self) -> &VulkanContext {
        self.context_ptr.as_ref()
    }

    pub unsafe fn device(&self) -> &Device {
        self.device.as_ref()
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl VulkanRenderable {
    /// Given a Window, creates a Vulkan Surface, Logical Device, and Render Pipelines, and wraps them together.
    pub fn new(context: &VulkanContext, window: WindowHandle, vsync: bool) -> Aliased<Self> {
        let context_ptr = NonNull::from(context);

        let surface = Surface::new(context, window);

        let device = Device::new(context, &surface);
        let device_ref = unsafe { device.as_ref() };
        let physical_ref = unsafe { device_ref.selection.physical() };

        let ds_info = DeviceSurfaceInfo::new(physical_ref, &surface);

        let buffers = Buffers::new(device_ref);

        let command_pool = CommandPool::new(device_ref);

        let default_texture = Texture::new(1, 1);
        let atlas = Some(Atlas::new(device_ref, &default_texture));

        let shaders = Shaders::compile(device_ref, &surface, &ds_info);

        let frames = Frames::collect(
            device_ref,
            &surface,
            &ds_info,
            &shaders.render_pass,
            &command_pool,
            vsync,
        );
        let frame_idx = 0;

        let this = Self {
            frame_idx,
            frames,
            atlas,
            command_pool,
            buffers,
            shaders,
            ds_info,
            device,
            surface,
            context_ptr,
        };

        this.texture_atlas(this.atlas.as_ref().unwrap(), &default_texture);

        Aliased::new(this)
    }
}

impl Drop for VulkanRenderable {
    fn drop(&mut self) {
        let res = unsafe { self.device().ash_device().device_wait_idle() };
        res.unwrap();
    }
}

// ================================================================================================================================ //
