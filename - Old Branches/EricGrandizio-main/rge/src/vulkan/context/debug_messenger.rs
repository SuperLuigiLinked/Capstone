/*
 *  Crate: RGE
 * Module: Vulkan - Context - Debug Messenger
 */

//! Internal utilities for receiving Debug Messages from Vulkan Drivers / Validation Layers.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkDebugUtilsMessengerEXT`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html>
pub struct DebugMessenger {
    /// Inner `VkDebugUtilsMessengerEXT`.
    pub messenger: vk::DebugUtilsMessengerEXT,

    /// Pointer to the object responsible for freeing this resource.
    ext_ptr: NonNull<ext::DebugUtils>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl DebugMessenger {
    pub unsafe fn ext(&self) -> &ext::DebugUtils {
        self.ext_ptr.as_ref()
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
impl DebugMessenger {
    /// Registers a Debug Callback and creates a new Vulkan Debug Messenger
    pub fn new(exts: &InstanceExts) -> Self {
        let ext_ptr = NonNull::from(&exts.debug_utils);

        #[rustfmt::skip]
        let severity_flags
        = vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
        | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
        ;

        #[rustfmt::skip]
        let type_flags
        = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        ;

        let create_info = vk::DebugUtilsMessengerCreateInfoEXT {
            message_severity: severity_flags,
            message_type: type_flags,
            pfn_user_callback: Some(Self::debug_callback),
            ..Default::default()
        };
        let res = unsafe {
            exts.debug_utils
                .create_debug_utils_messenger(&create_info, None)
        };

        let messenger = res.unwrap();

        Self { messenger, ext_ptr }
    }
}

impl Drop for DebugMessenger {
    fn drop(&mut self) {
        unsafe {
            self.ext()
                .destroy_debug_utils_messenger(self.messenger, None)
        };
    }
}

// ================================================================================================================================ //

#[allow(unused_variables)]
impl DebugMessenger {
    /// Callback function that logs Debug Messages.\
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html>
    unsafe extern "system" fn debug_callback(
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        user_data: *mut c_void,
    ) -> u32 {
        // Do not let any PANICs escape.
        let _ = std::panic::catch_unwind(move || {
            let msg_severity = match message_severity {
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "ERROR",
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "WARNING",
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "INFO",
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "VERBOSE",
                _ => unreachable!(),
            };
            let msg_type = match message_type {
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "GENERAL",
                vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
                vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
                _ => unreachable!(),
            };

            let (id, msg) = if let Some(data) = unsafe { callback_data.as_ref() } {
                let id = unsafe { CStr::from_ptr(data.p_message_id_name) }.to_string_lossy();
                let msg = unsafe { CStr::from_ptr(data.p_message) }.to_string_lossy();
                (id, msg)
            } else {
                let id = Cow::Borrowed("???");
                let msg = Cow::Borrowed("");
                (id, msg)
            };

            eprintln!("\n[VULKAN DEBUG - {msg_type} {msg_severity}] <{id}>\n{msg}");
        });

        vk::FALSE
    }
}

// ================================================================================================================================ //
