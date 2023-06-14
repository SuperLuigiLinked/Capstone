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
pub struct Shader {
    /// Inner `VkShaderModule`.
    pub module: vk::ShaderModule,

    /// Pointer to the object responsible for freeing this resource.
    device_ptr: NonNull<Device>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Shader {
    pub unsafe fn device(&self) -> &Device {
        self.device_ptr.as_ref()
    }

    pub unsafe fn ash_device(&self) -> &ash::Device {
        &self.device().logical.inner
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Shader {
    /// Creates a Shader Module from the compiled Shader.
    pub fn new(device: &Device, code: &shaderc::CompilationArtifact) -> Self {
        let device_ptr = NonNull::from(device);

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

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.ash_device().destroy_shader_module(self.module, None) };
    }
}

// ================================================================================================================================ //

/// A group of related Shader Modules.
pub struct Shaders {
    /// Fragment Shader.
    pub fragment: Shader,

    /// Vertex Shader.
    pub vertex: Shader,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Shaders {
    /// Compiles all the Shader Programs specified in the `sources` module.
    pub fn compile(device: &Device) -> Self {
        let compiler = shaderc::Compiler::new().unwrap();
        let options = shaderc::CompileOptions::new().unwrap();

        let fragment = Self::compile_spirv(&compiler, &options, device, &FRAGMENT);
        let vertex = Self::compile_spirv(&compiler, &options, device, &VERTEX);

        Self { fragment, vertex }
    }

    /// Compiles a single Shader Program from the given source.
    fn compile_spirv(
        compiler: &shaderc::Compiler,
        options: &shaderc::CompileOptions,
        device: &Device,
        source: &ShaderSource,
    ) -> Shader {
        let artifact = compiler
            .compile_into_spirv(
                source.text,
                source.kind,
                source.name,
                &sources::ENTRY_POINT.as_str(),
                Some(options),
            )
            .unwrap();

        Shader::new(device, &artifact)
    }
}

// ================================================================================================================================ //
