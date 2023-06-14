/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Device
 */

//! Internal utilities for managing Vulkan Devices.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// A collect
pub struct Device {
    /// Device Extension Function-loaders.
    pub exts: Aliased<DeviceExts>,

    /// Wrapped Logical Device.
    pub logical: LogicalDevice,

    /// Associated Physical Device and Queue Families.
    pub selection: DeviceSelection,

    /// Pointer to the object that holds the resources referenced by the selected indices.
    context_ptr: NonNull<VulkanContext>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Device {
    pub unsafe fn exts(&self) -> &DeviceExts {
        self.exts.as_ref()
    }

    pub unsafe fn context(&self) -> &VulkanContext {
        self.context_ptr.as_ref()
    }

    pub unsafe fn instance(&self) -> &Instance {
        self.context().instance()
    }

    pub fn ash_device(&self) -> &ash::Device {
        &self.logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Device {
    /// Creates a Device and associated Device-state for the given Surface.
    pub fn new(context: &VulkanContext, surface: &Surface) -> Aliased<Self> {
        let context_ptr = NonNull::from(context);

        let instance_ref = unsafe { &context.instance().inner };

        let selection = DeviceSelection::new(context, surface);

        let logical = LogicalDevice::new(instance_ref, &selection);

        let device_ref = &logical.inner;

        let exts = DeviceExts::new(instance_ref, device_ref);

        let this = Self {
            exts,
            logical,
            selection,
            context_ptr,
        };
        Aliased::new(this)
    }
}

// ================================================================================================================================ //
