/*
 *  Crate: RGE
 * Module: Vulkan - Context
 */

//! Internal utilities for managing Context-State in Vulkan.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

mod instance;
pub use instance::*;

mod instance_exts;
pub use instance_exts::*;

mod physical_device;
pub use physical_device::*;

mod debug_messenger;
pub use debug_messenger::*;

// ================================================================================================================================ //

/// All Vulkan-related State that exists independent of any renderable resources.
pub struct VulkanContext {
    /// List of available Physical Devices to use.
    pub devices: Vec<PhysicalDevice>,

    #[cfg(debug_assertions)]
    /// Debug-only Callback state.
    pub debug: DebugMessenger,

    /// Instance Extension Function-loaders.
    pub exts: Aliased<InstanceExts>,

    /// The Vulkan Instance.
    pub instance: Aliased<Instance>,

    /// Vulkan Function-loader.
    pub entry: Aliased<ash::Entry>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl VulkanContext {
    pub unsafe fn entry(&self) -> &ash::Entry {
        self.entry.as_ref()
    }

    pub unsafe fn instance(&self) -> &Instance {
        self.instance.as_ref()
    }

    pub unsafe fn exts(&self) -> &InstanceExts {
        self.exts.as_ref()
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl VulkanContext {
    /// Queries and wraps all available Vulkan Context state.
    pub fn new() -> Aliased<Self> {
        let entry = Aliased::new(ash::Entry::linked());
        let entry_ref = unsafe { entry.as_ref() };

        let instance = Instance::new(entry_ref);
        let instance_ref = unsafe { &instance.as_ref().inner };

        let exts = InstanceExts::new(entry_ref, instance_ref);
        let _exts_ref = unsafe { exts.as_ref() };

        #[cfg(debug_assertions)]
        let debug = DebugMessenger::new(_exts_ref);

        let devices = PhysicalDevice::collect(instance_ref);
        assert!(!devices.is_empty());

        let this = Self {
            devices,
            #[cfg(debug_assertions)]
            debug,
            exts,
            instance,
            entry,
        };

        Aliased::new(this)
    }
}

// ================================================================================================================================ //
