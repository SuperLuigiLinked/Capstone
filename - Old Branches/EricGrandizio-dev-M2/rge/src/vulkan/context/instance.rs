/*
 *  Crate: RGE
 * Module: Vulkan - Context - Instance
 */

//! Internal utilities for managing Vulkan Instances.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkInstance`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstance.html>
pub struct Instance {
    /// Inner `ash` `VkInstance`.
    pub inner: ash::Instance,

    /// Layers supported by this Instance.
    pub layers: Layers,

    /// Extensions supported by this Instance.
    pub extensions: Extensions,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Instance {
    /// Queries the Supported Extensions/Layers and creates a Vulkan Instance.
    pub fn new(entry: &ash::Entry) -> Aliased<Self> {
        let extensions = Extensions::supported_instance(entry);
        let required_extensions = Extensions::required_instance();
        let unsupported_extensions: Vec<_> =
            extensions.unsupported(required_extensions.iter()).collect();
        assert!(unsupported_extensions.is_empty());

        let layers = Layers::supported_instance(entry);
        let required_layers = Layers::required_instance();
        let unsupported_layers: Vec<_> = layers.unsupported(required_layers.iter()).collect();
        assert!(unsupported_layers.is_empty());

        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: required_extensions.as_ptr() as *const *const c_char,
            enabled_extension_count: required_extensions.len() as u32,
            pp_enabled_layer_names: required_layers.as_ptr() as *const *const c_char,
            enabled_layer_count: required_layers.len() as u32,
            ..Default::default()
        };
        let inner = unsafe { entry.create_instance(&create_info, None) }.unwrap();

        let this = Self {
            inner,
            extensions,
            layers,
        };
        Aliased::new(this)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.inner.destroy_instance(None) };
    }
}

// ================================================================================================================================ //
