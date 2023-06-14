/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Pipelines
 */

//! The all-encompassing module where an entire Render Pipeline is created for the Shaders, from start to finish.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use glsl::VulkanFormat;

// ================================================================================================================================ //

/// All state required for Shader Pipelines.
pub struct ShaderPipelines {
    /// Graphics Pipelines for each type of primitive.
    pub graphics: GraphicsPipelines,

    /// Layout of all the Graphics Pipelines.
    pub layout: PipelineLayout,

    /// Render Pass for all the Graphics Pipelines.
    pub render_pass: RenderPass,
}

// ================================================================================================================================ //

impl ShaderPipelines {
    /// Creates the Pipelines required to run the Shader Programs.
    pub fn new(
        device: &Device,
        surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        shaders: &Shaders,
    ) -> Self {
        // -------------------------------------------------------------------------------------------------------------------------------- //

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

        let render_pass = RenderPass::new(device, &render_pass_info);

        // -------------------------------------------------------------------------------------------------------------------------------- //

        let pc_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX,
            offset: 0,
            size: size_of::<glsl::PushConstants>() as u32,
        };

        let layout_info = vk::PipelineLayoutCreateInfo {
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 0,
            p_set_layouts: null(),
            push_constant_range_count: 1,
            p_push_constant_ranges: addr_of!(pc_range),
            ..Default::default()
        };

        let layout = PipelineLayout::new(device, &pc_range, &layout_info);

        // -------------------------------------------------------------------------------------------------------------------------------- //

        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_info = vk::PipelineDynamicStateCreateInfo {
            flags: vk::PipelineDynamicStateCreateFlags::empty(),
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
            ..Default::default()
        };

        let vertex_stage = vk::PipelineShaderStageCreateInfo {
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            stage: vk::ShaderStageFlags::VERTEX,
            module: shaders.vertex.module,
            p_name: shaders::ENTRY_POINT.as_ptr(),
            p_specialization_info: null(),
            ..Default::default()
        };
        let fragment_stage = vk::PipelineShaderStageCreateInfo {
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            module: shaders.fragment.module,
            stage: vk::ShaderStageFlags::FRAGMENT,
            p_name: shaders::ENTRY_POINT.as_ptr(),
            p_specialization_info: null(),
            ..Default::default()
        };
        let shader_stages = [vertex_stage, fragment_stage];

        let binding_vertex = vk::VertexInputBindingDescription {
            binding: 0,
            stride: size_of::<glsl::Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        };
        let bindings = [binding_vertex];

        /// Type used by Vertex's XYZW field.
        #[allow(non_camel_case_types)]
        type GLSL_XYZW = glsl::vec4;

        /// Type used by Vertex's RGBA field.
        #[allow(non_camel_case_types)]
        type GLSL_RGBA = glsl::vec4;

        let attribute_xyzw = vk::VertexInputAttributeDescription {
            location: 0,
            binding: binding_vertex.binding,
            format: GLSL_XYZW::VK_FORMAT,
            offset: 0,
        };
        let attribute_rgba = vk::VertexInputAttributeDescription {
            location: attribute_xyzw.location + 1,
            binding: binding_vertex.binding,
            format: GLSL_RGBA::VK_FORMAT,
            offset: attribute_xyzw.offset + size_of::<GLSL_XYZW>() as u32,
        };

        let attributes = [attribute_xyzw, attribute_rgba];

        let vertex_info = vk::PipelineVertexInputStateCreateInfo {
            flags: vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: bindings.len() as u32,
            p_vertex_binding_descriptions: bindings.as_ptr(),
            vertex_attribute_description_count: attributes.len() as u32,
            p_vertex_attribute_descriptions: attributes.as_ptr(),
            ..Default::default()
        };

        let point_list_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::POINT_LIST,
            primitive_restart_enable: vk::FALSE,
            ..Default::default()
        };
        let line_list_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::LINE_LIST,
            primitive_restart_enable: vk::FALSE,
            ..Default::default()
        };
        let tri_list_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
            ..Default::default()
        };
        let line_strip_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::LINE_STRIP,
            primitive_restart_enable: vk::TRUE,
            ..Default::default()
        };
        let tri_strip_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_STRIP,
            primitive_restart_enable: vk::TRUE,
            ..Default::default()
        };
        let tri_fan_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_FAN,
            primitive_restart_enable: vk::TRUE,
            ..Default::default()
        };

        let (viewport, scissor) = ds_info.ideal_viewport_scissor(surface);

        let viewport_info = vk::PipelineViewportStateCreateInfo {
            flags: vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: 1,
            p_viewports: addr_of!(viewport),
            scissor_count: 1,
            p_scissors: addr_of!(scissor),
            ..Default::default()
        };

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo {
            flags: vk::PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
            ..Default::default()
        };

        let multisample_info = vk::PipelineMultisampleStateCreateInfo {
            flags: vk::PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 0.0,
            p_sample_mask: null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
            ..Default::default()
        };

        let blend_attachment = vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        };
        let blend_info = vk::PipelineColorBlendStateCreateInfo {
            flags: vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::CLEAR,
            attachment_count: 1,
            p_attachments: addr_of!(blend_attachment),
            blend_constants: [0.0, 0.0, 0.0, 0.0],
            ..Default::default()
        };

        let default_pipeline_info = vk::GraphicsPipelineCreateInfo {
            flags: vk::PipelineCreateFlags::empty(),
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_info,
            p_input_assembly_state: null(),
            p_tessellation_state: null(),
            p_viewport_state: &viewport_info,
            p_rasterization_state: &rasterizer_info,
            p_multisample_state: &multisample_info,
            p_depth_stencil_state: null(),
            p_color_blend_state: &blend_info,
            p_dynamic_state: &dynamic_info,
            layout: layout.handle,
            render_pass: render_pass.handle,
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: 0,
            ..Default::default()
        };

        let point_list_info = vk::GraphicsPipelineCreateInfo {
            p_input_assembly_state: &point_list_assembly_info,
            ..default_pipeline_info
        };
        let line_list_info = vk::GraphicsPipelineCreateInfo {
            p_input_assembly_state: &line_list_assembly_info,
            ..default_pipeline_info
        };
        let tri_list_info = vk::GraphicsPipelineCreateInfo {
            p_input_assembly_state: &tri_list_assembly_info,
            ..default_pipeline_info
        };
        let line_strip_info = vk::GraphicsPipelineCreateInfo {
            p_input_assembly_state: &line_strip_assembly_info,
            ..default_pipeline_info
        };
        let tri_strip_info = vk::GraphicsPipelineCreateInfo {
            p_input_assembly_state: &tri_strip_assembly_info,
            ..default_pipeline_info
        };
        let tri_fan_info = vk::GraphicsPipelineCreateInfo {
            p_input_assembly_state: &tri_fan_assembly_info,
            ..default_pipeline_info
        };

        let graphics = GraphicsPipelines::new(
            device,
            &[
                point_list_info,
                line_list_info,
                tri_list_info,
                line_strip_info,
                tri_strip_info,
                tri_fan_info,
            ],
        );

        // -------------------------------------------------------------------------------------------------------------------------------- //

        Self {
            graphics,
            layout,
            render_pass,
        }
    }
}

// ================================================================================================================================ //
