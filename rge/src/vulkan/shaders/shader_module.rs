/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Shader
 */

//! Internal utilities for managing Vulkan Shader Modules.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkShaderModule`.\
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html>
pub struct ShaderModule {
    /// Inner `VkShaderModule`.
    pub module: vk::ShaderModule,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl ShaderModule {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl ShaderModule {
    /// Compiles a Shader Module.
    pub fn compile(
        device: &Device,
        compiler: &shaderc::Compiler,
        options: &shaderc::CompileOptions,
        source: &sources::ShaderSource,
    ) -> Self {
        let device_ptr = NonNull::from(device);

        let res = compiler.compile_into_spirv(
            source.text,
            source.kind,
            source.name,
            &sources::ENTRY_POINT.as_str(),
            Some(options),
        );

        let code = res.unwrap();

        let create_info = vk::ShaderModuleCreateInfo {
            p_code: code.as_binary().as_ptr(),
            code_size: code.len(),
            ..Default::default()
        };

        let res = unsafe { device.ash_device().create_shader_module(&create_info, None) };
        let module = res.unwrap();

        Self { module, device_ptr }
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_shader_module(self.module, None) };
    }
}

// ================================================================================================================================ //
