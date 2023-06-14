/*
 *  Crate: RGE
 * Module: Vulkan - Utils - Layers
 */

//! Internal utilities for enumerating Required and Supported Vulkan Layers for Instances and Devices.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// A list of Supported Layers.
pub struct Layers {
    /// The Properties of each Layer.
    pub list: Vec<vk::LayerProperties>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Layers {
    /// Collects the Supported Layers for Vulkan Instances.
    pub fn supported_instance(entry: &ash::Entry) -> Self {
        let list = entry.enumerate_instance_layer_properties().unwrap();
        Self { list }
    }

    /// Collects the Supported Layers for a given Vulkan Instance + Physical Device.
    pub fn supported_device(instance: &ash::Instance, device: vk::PhysicalDevice) -> Self {
        let res = unsafe { instance.enumerate_device_layer_properties(device) };

        let list = res.unwrap();

        Self { list }
    }

    /// Collects the Names of Required Instance Layers.
    pub fn required_instance() -> Vec<NtString<'static>> {
        vec![VALIDATION]
    }

    /// Collects the Names of Required Device Layers.
    pub fn required_device() -> Vec<NtString<'static>> {
        vec![VALIDATION]
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Layers {
    /// Returns an Iterator over the Names of each Supported Layer.
    pub fn names(&self) -> impl Iterator<Item = NtString> {
        self.list.iter().map(|layer| {
            let name = &layer.layer_name;
            NtString::from_chars_cap(name, name.len())
        })
    }

    /// Returns whether or not the Layer provided by `name` is supported.
    pub fn supports(&self, name: &NtString) -> bool {
        self.names().any(|sup_name| sup_name == *name)
    }

    /// Returns whether or not the Layers provided by `names` are supported.
    pub fn supports_all(&self, names: &[NtString]) -> bool {
        names.iter().all(|name| self.supports(name))
    }

    #[allow(unused)]
    /// Returns an Iterator over the Supported Names.
    pub fn supported<'a>(
        &'a self,
        names: impl Iterator<Item = &'a NtString<'a>>,
    ) -> impl Iterator<Item = &'a NtString<'a>> {
        names.filter(|name| self.supports(name))
    }

    /// Returns an Iterator over the Unsupported Names.
    pub fn unsupported<'a>(
        &'a self,
        names: impl Iterator<Item = &'a NtString<'a>>,
    ) -> impl Iterator<Item = &'a NtString<'a>> {
        names.filter(|name| !self.supports(name))
    }
}

// ================================================================================================================================ //

use names::*;

/// Constants for Layer Names.
#[allow(unused)]
mod names {
    use super::*;

    // ---------------------------------------------------------------- //

    /// <https://vulkan.lunarg.com/doc/view/1.2.170.0/linux/khronos_validation_layer.html>
    pub const VALIDATION: NtString = NtString::from_bytes(b"VK_LAYER_KHRONOS_validation\0");

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //
