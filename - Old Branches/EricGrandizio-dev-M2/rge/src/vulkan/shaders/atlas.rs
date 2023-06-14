/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Atlas
 */

//! Internal utilities for creating a Texture Atlas in Device Memory to be used in shaders.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// A device-local Texture Atlas.
pub struct Atlas {
    /// Texture Sampler.
    pub sampler: Sampler,

    /// Image View.
    pub view: ImageView,

    /// Image Buffer.
    pub image: Image,

    /// Device Memory.
    pub memory: DeviceMemory,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Atlas {
    /// Creates a new Atlas from the given Texture.
    pub fn new(device: &Device, texture: &Texture) -> Self {
        assert!(!texture.is_empty(), "Texture should be of non-zero size");

        // ---------------------------------------------------------------- //

        let tex_extent = texture.vk_extent();
        let image_format = vk::Format::R8G8B8A8_SRGB;

        // ---------------------------------------------------------------- //

        let image_info = vk::ImageCreateInfo {
            image_type: vk::ImageType::TYPE_2D,
            extent: tex_extent,
            mip_levels: 1,
            array_layers: 1,
            format: image_format,
            tiling: vk::ImageTiling::OPTIMAL,
            initial_layout: vk::ImageLayout::UNDEFINED,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };
        let mut image = Image::new(device, &image_info);

        // ---------------------------------------------------------------- //

        let memory = DeviceMemory::new(
            device,
            image.mem_reqs.size,
            image.mem_reqs.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );

        unsafe {
            image.bind(device, &memory, 0);
        }

        // ---------------------------------------------------------------- //

        let view_info = vk::ImageViewCreateInfo {
            image: image.handle,
            view_type: vk::ImageViewType::TYPE_2D,
            format: image_format,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        let view = ImageView::new(device, &view_info);

        // ---------------------------------------------------------------- //

        let sampler_info = vk::SamplerCreateInfo {
            flags: vk::SamplerCreateFlags::empty(),
            unnormalized_coordinates: vk::FALSE,
            address_mode_u: vk::SamplerAddressMode::REPEAT,
            address_mode_v: vk::SamplerAddressMode::REPEAT,
            address_mode_w: vk::SamplerAddressMode::REPEAT,
            border_color: vk::BorderColor::INT_TRANSPARENT_BLACK,
            min_filter: vk::Filter::NEAREST,
            mag_filter: vk::Filter::NEAREST,
            mipmap_mode: vk::SamplerMipmapMode::NEAREST,
            mip_lod_bias: 0.0,
            min_lod: 0.0,
            max_lod: 0.0,
            anisotropy_enable: vk::FALSE,
            max_anisotropy: 1.0,
            compare_enable: vk::FALSE,
            compare_op: vk::CompareOp::ALWAYS,
            ..Default::default()
        };
        let sampler = Sampler::new(device, &sampler_info);

        // ---------------------------------------------------------------- //

        Self {
            sampler,
            view,
            image,
            memory,
        }
    }
}

// ================================================================================================================================ //

/// Wrapper for a `VkImage`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImage.html>
pub struct Image {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html>
    pub mem_reqs: vk::MemoryRequirements,

    /// Inner `VkImage`.
    pub handle: vk::Image,

    /// Byte-Size of this Image.
    pub size: vk::DeviceSize,

    /// Byte-Offset of this Image.
    pub offs: vk::DeviceSize,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Image {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Image {
    /// Creates a new Image.
    pub fn new(device: &Device, create_info: &vk::ImageCreateInfo) -> Self {
        let device_ptr = NonNull::from(device);

        let w = vk::DeviceSize::try_from(create_info.extent.width).unwrap();
        let h = vk::DeviceSize::try_from(create_info.extent.height).unwrap();
        let d = vk::DeviceSize::try_from(create_info.extent.depth).unwrap();
        let elem_size = size_of::<RGBA>() as vk::DeviceSize;
        let size = elem_size
            .checked_mul(w)
            .unwrap()
            .checked_mul(h)
            .unwrap()
            .checked_mul(d)
            .unwrap();
        let offs = 0;

        let res = unsafe { device.ash_device().create_image(create_info, None) };
        let handle = res.unwrap();

        let mem_reqs = unsafe { device.ash_device().get_image_memory_requirements(handle) };

        Self {
            mem_reqs,
            handle,
            size,
            offs,
            device_ptr,
        }
    }

    /// Binds the Image to the Device Memory at the given offset.
    pub unsafe fn bind(&mut self, device: &Device, memory: &DeviceMemory, offset: vk::DeviceSize) {
        let res = device
            .ash_device()
            .bind_image_memory(self.handle, memory.handle, offset);

        res.unwrap();

        self.offs = offset;
    }

    #[allow(unused)]
    /// Offset of the End of this Buffer.
    pub fn end_offs(&self) -> vk::DeviceSize {
        self.offs + self.size
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_image(self.handle, None) };
    }
}

// ================================================================================================================================ //

/// Wrapper for a `VkImageView`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageView.html>
pub struct ImageView {
    /// Inner `VkImageView`.
    pub handle: vk::ImageView,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl ImageView {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl ImageView {
    /// Creates a new Image View.
    pub fn new(device: &Device, create_info: &vk::ImageViewCreateInfo) -> Self {
        let device_ptr = NonNull::from(device);

        let res = unsafe { device.ash_device().create_image_view(create_info, None) };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_image_view(self.handle, None) };
    }
}

// ================================================================================================================================ //

/// Wrapper for a `VkSampler`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampler.html>
pub struct Sampler {
    /// Inner `VkSampler`.
    pub handle: vk::Sampler,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Sampler {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Sampler {
    /// Creates a new Sampler.
    pub fn new(device: &Device, create_info: &vk::SamplerCreateInfo) -> Self {
        let device_ptr = NonNull::from(device);

        let res = unsafe { device.ash_device().create_sampler(create_info, None) };
        let handle = res.unwrap();

        Self { handle, device_ptr }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            let ash_device = self.ash_device();

            // Make sure the Sampler isn't being Used currently.
            let res = ash_device.device_wait_idle();
            res.unwrap();

            ash_device.destroy_sampler(self.handle, None);
        }
    }
}

// ================================================================================================================================ //
