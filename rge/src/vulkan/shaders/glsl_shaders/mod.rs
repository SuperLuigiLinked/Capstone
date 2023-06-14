/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - GLSL Shaders
 */

//! The all-encompassing module where an entire Render Pipeline is created for the RGBA Shaders, from start to finish.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

pub mod rgba;
pub mod uv;

pub use rgba::Shader as ShaderRGBA;
pub use rgba::Vertex;

pub use uv::Shader as ShaderUV;
pub use uv::Vertex as VertexUV;

// ================================================================================================================================ //

/// A group of related Shader Modules.
pub struct Shaders {
    /// UV-Shader.
    pub uv: ShaderUV,

    /// RGBA-Shader.
    pub rgba: ShaderRGBA,

    /// Render Pass for all the Graphics Pipelines.
    pub render_pass: RenderPass,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Shaders {
    /// Compiles all the Shader Programs.
    pub fn compile(device: &Device, surface: &Surface, ds_info: &DeviceSurfaceInfo) -> Self {
        let render_pass = Self::create_render_pass(device, ds_info);

        let compiler = shaderc::Compiler::new().unwrap();

        let rgba = ShaderRGBA::new(device, surface, ds_info, &render_pass, &compiler);
        let uv = ShaderUV::new(device, surface, ds_info, &render_pass, &compiler);

        Self {
            uv,
            rgba,
            render_pass,
        }
    }

    /// Creates the Render Pass shared by all Shaders.
    fn create_render_pass(device: &Device, ds_info: &DeviceSurfaceInfo) -> RenderPass {
        let surface_format = ds_info.ideal_format();

        let attachment_desc = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: surface_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        };

        let attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass_desc = vk::SubpassDescription {
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: null(),
            color_attachment_count: 1,
            p_color_attachments: addr_of!(attachment_ref),
            p_resolve_attachments: null(),
            p_depth_stencil_attachment: null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: null(),
        };

        let subpass_dep = vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        };

        let render_pass_info = vk::RenderPassCreateInfo {
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: 1,
            p_attachments: addr_of!(attachment_desc),
            subpass_count: 1,
            p_subpasses: addr_of!(subpass_desc),
            dependency_count: 1,
            p_dependencies: addr_of!(subpass_dep),
            ..Default::default()
        };

        RenderPass::new(device, &render_pass_info)
    }
}

// ================================================================================================================================ //
