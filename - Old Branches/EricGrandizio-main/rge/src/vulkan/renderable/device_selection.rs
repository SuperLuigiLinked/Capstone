/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Device Selection
 */

//! Internal utilities for Selecting a Physical Device to use for Rendering.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// An (Index, Rank) pair.\
/// A rank of 0 indicates an Invalid value.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ranking {
    /// Index.
    pub idx: usize,

    /// Ranking. (Greater is Better)
    pub rank: usize,
}

impl PartialOrd for Ranking {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.rank).partial_cmp(&other.rank)
    }
}

impl Ord for Ranking {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.rank).cmp(&other.rank)
    }
}

// ================================================================================================================================ //

/// A collection of parameters to select a Physical Device for Logical Device creation.
pub struct DeviceSelection {
    /// Index of the Present-Queue Family.
    pub present_idx: usize,

    /// Index of the Graphics-Queue Family.
    pub graphics_idx: usize,

    /// Index of the Physical Device.
    pub physical_idx: usize,

    /// Pointer to the object that holds the resources referenced by the selected indices.
    context_ptr: NonNull<VulkanContext>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl DeviceSelection {
    pub unsafe fn context(&self) -> &VulkanContext {
        self.context_ptr.as_ref()
    }

    pub unsafe fn physical(&self) -> &PhysicalDevice {
        self.context().devices.get(self.physical_idx).unwrap()
    }

    pub unsafe fn graphics_family(&self) -> &vk::QueueFamilyProperties {
        self.physical().queues.get(self.graphics_idx).unwrap()
    }

    pub unsafe fn present_family(&self) -> &vk::QueueFamilyProperties {
        self.physical().queues.get(self.present_idx).unwrap()
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl DeviceSelection {
    /// Given a list of a Physical Devices, picks the best one to use to render to the given Window Surface.
    pub fn new(context: &VulkanContext, surface: &Surface) -> Self {
        let context_ptr = NonNull::from(context);

        let physical_ranks = Self::device_rankings(&context.devices, surface);
        let physical_best = physical_ranks.max().unwrap();
        let physical_idx = physical_best.idx;
        let physical_ref = context.devices.get(physical_idx).unwrap();

        let graphics_ranks = Self::graphics_queue_rankings(physical_ref, surface);
        let graphics_best = graphics_ranks.max().unwrap();
        let graphics_idx = graphics_best.idx;
        assert!(physical_ref.queues.get(graphics_idx).is_some());

        let present_ranks = Self::present_queue_rankings(physical_ref, surface);
        let present_best = present_ranks.max().unwrap();
        let present_idx = present_best.idx;
        assert!(physical_ref.queues.get(present_idx).is_some());

        Self {
            present_idx,
            graphics_idx,
            physical_idx,
            context_ptr,
        }
    }
}

// ================================================================================================================================ //

/// Miscellaneous functionality for helping Swapchain Creation: <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html>
impl DeviceSelection {
    /// Whether or not all Queue-Families indices are unique.
    pub fn queues_unique(&self) -> bool {
        self.graphics_idx != self.present_idx
    }

    /// An array of all Queue-Family indices.
    pub fn queue_indices(&self) -> [u32; 2] {
        [self.graphics_idx as u32, self.present_idx as u32]
    }

    /// Count of concurrent Queue-Families.
    pub fn concurrent_queue_count(&self) -> u32 {
        if self.queues_unique() {
            2
        } else {
            0
        }
    }

    /// Count of unique Queue-Families.
    pub fn unique_queue_count(&self) -> u32 {
        if self.queues_unique() {
            2
        } else {
            1
        }
    }

    /// Sharing Mode for Queue-Families.
    pub fn queue_mode(&self) -> vk::SharingMode {
        if self.queues_unique() {
            vk::SharingMode::CONCURRENT
        } else {
            vk::SharingMode::EXCLUSIVE
        }
    }
}

// ================================================================================================================================ //

impl DeviceSelection {
    /// Ranks a Physical Device.\
    /// It must support all the required Extensions and Layers.\
    /// It must support Graphics and Present operations.
    /// Higher-End GPUs are prefered over CPUs.
    fn rank_device(
        device: &PhysicalDevice,
        _surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        required_extensions: &[NtString],
        required_layers: &[NtString],
    ) -> usize {
        if !device.extensions.supports_all(required_extensions) {
            return 0;
        }

        if !device.layers.supports_all(required_layers) {
            return 0;
        }

        if (ds_info.formats.is_empty()) || (ds_info.modes.is_empty()) {
            return 0;
        }

        {
            let mut rank = 1;

            match device.properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => rank += 50,
                vk::PhysicalDeviceType::INTEGRATED_GPU => rank += 40,
                vk::PhysicalDeviceType::VIRTUAL_GPU => rank += 30,
                vk::PhysicalDeviceType::CPU => rank += 20,
                vk::PhysicalDeviceType::OTHER => rank += 10,
                _ => unreachable!(),
            }

            rank
        }
    }

    /// Ranks a Queue Family.\
    /// It must support Graphics operations.
    fn rank_queue_graphics(
        _device: &PhysicalDevice,
        _surface: &Surface,
        _ds_info: &DeviceSurfaceInfo,
        queue_props: &vk::QueueFamilyProperties,
        _queue_idx: usize,
    ) -> usize {
        let supports_graphics = queue_props.queue_flags.contains(vk::QueueFlags::GRAPHICS);

        if !supports_graphics {
            return 0;
        }

        {
            let mut rank = 1;

            rank += queue_props.queue_count as usize;

            rank
        }
    }

    /// Ranks a Queue Family.\
    /// It must support Present operations.
    fn rank_queue_present(
        _device: &PhysicalDevice,
        _surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        queue_props: &vk::QueueFamilyProperties,
        queue_idx: usize,
    ) -> usize {
        let supports_present = ds_info.support.get(queue_idx).unwrap();

        if !supports_present {
            return 0;
        }

        {
            let mut rank = 1;

            rank += queue_props.queue_count as usize;

            rank
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Returns an iterator over the `Ranking`s of all the provided Physical Devices.
    fn device_rankings<'a>(
        devices: &'a [PhysicalDevice],
        surface: &'a Surface,
    ) -> impl Iterator<Item = Ranking> + 'a {
        let required_extensions = Extensions::required_device();
        let required_layers = Layers::required_device();

        let get_ranking = move |(idx, device): (usize, &PhysicalDevice)| {
            let ds_info = DeviceSurfaceInfo::new(device, surface);

            let rank = Self::rank_device(
                device,
                surface,
                &ds_info,
                &required_extensions,
                &required_layers,
            );
            Ranking { idx, rank }
        };

        devices
            .iter()
            .enumerate()
            .map(get_ranking)
            .filter(|ranking| ranking.rank != 0)
    }

    /// Returns an iterator over the Graphics `Ranking`s of all the provided Queue Families.
    fn graphics_queue_rankings<'a>(
        device: &'a PhysicalDevice,
        surface: &'a Surface,
    ) -> impl Iterator<Item = Ranking> + 'a {
        let ds_info = DeviceSurfaceInfo::new(device, surface);

        let get_ranking = move |(idx, queue_props): (usize, &vk::QueueFamilyProperties)| {
            let rank = Self::rank_queue_graphics(device, surface, &ds_info, queue_props, idx);
            Ranking { idx, rank }
        };

        device
            .queues
            .iter()
            .enumerate()
            .map(get_ranking)
            .filter(|ranking| ranking.rank != 0)
    }

    /// Returns an iterator over the Present `Ranking`s of all the provided Queue Families.
    fn present_queue_rankings<'a>(
        device: &'a PhysicalDevice,
        surface: &'a Surface,
    ) -> impl Iterator<Item = Ranking> + 'a {
        let ds_info = DeviceSurfaceInfo::new(device, surface);

        let get_ranking = move |(idx, queue_props): (usize, &vk::QueueFamilyProperties)| {
            let rank = Self::rank_queue_present(device, surface, &ds_info, queue_props, idx);
            Ranking { idx, rank }
        };

        device
            .queues
            .iter()
            .enumerate()
            .map(get_ranking)
            .filter(|ranking| ranking.rank != 0)
    }
}

// ================================================================================================================================ //
