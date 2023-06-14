/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - UV
 */

//! Creates the entire Render Pipeline for the "UV-Shader", from start to finish.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use glsl::VulkanFormat;

use sources::uv as sources;

// ================================================================================================================================ //

/// A Vertex, used to represent points/colors on various shapes.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Vertex {
    /// Position of the Vertex. Only XY values are used for positioning. Z values represent Depth/Layer [TODO]. W values represent Point-Size.
    pub xyzw: VERTEX_XYZW,

    /// Color of the Vertex.
    pub rgba: VERTEX_RGBA,

    /// Texture-coordinates of the Vertex.
    pub uv: VERTEX_UV,
}

/// Type used by Vertex's XYZW field.
#[allow(non_camel_case_types)]
type VERTEX_XYZW = glsl::vec4;

/// Type used by Vertex's RGBA field.
#[allow(non_camel_case_types)]
type VERTEX_RGBA = glsl::vec4;

/// Type used by Vertex's UV field.
#[allow(non_camel_case_types)]
type VERTEX_UV = glsl::vec2;

// ================================================================================================================================ //

/// All state required for Shader Pipelines.
pub struct Shader {
    /// Graphics Pipelines for each type of primitive.
    pub graphics: GraphicsPipelines,

    /// Layout of all the Graphics Pipelines.
    pub pipeline_layout: PipelineLayout,

    /// Descriptor Sets.
    pub descriptor_set: vk::DescriptorSet,

    /// Pool for allocating Descriptor Sets.
    pub descriptor_pool: DescriptorPool,

    /// Layout of the Descriptor Set.
    pub descriptor_layout: DescriptorSetLayout,

    /// Fragment Shader.
    pub fragment: ShaderModule,

    /// Vertex Shader.
    pub vertex: ShaderModule,
}

// ================================================================================================================================ //

impl Shader {
    /// Creates the Pipelines required to run the Shader Programs.
    pub fn new(
        device: &Device,
        surface: &Surface,
        ds_info: &DeviceSurfaceInfo,
        render_pass: &RenderPass,
        compiler: &shaderc::Compiler,
    ) -> Self {
        // -------------------------------------------------------------------------------------------------------------------------------- //

        let options = shaderc::CompileOptions::new().unwrap();

        let vertex = ShaderModule::compile(device, compiler, &options, &sources::VERTEX);
        let fragment = ShaderModule::compile(device, compiler, &options, &sources::FRAGMENT);

        // -------------------------------------------------------------------------------------------------------------------------------- //

        let max_frames = 1;

        let sampler_binding = vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            p_immutable_samplers: null(),
            stage_flags: vk::ShaderStageFlags::FRAGMENT,
        };
        let descriptor_bindings = [sampler_binding];

        let sampler_pool_size = vk::DescriptorPoolSize {
            descriptor_count: max_frames,
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        };
        let descriptor_pool_sizes = [sampler_pool_size];

        let descriptor_layout_info = vk::DescriptorSetLayoutCreateInfo {
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: descriptor_bindings.len() as u32,
            p_bindings: descriptor_bindings.as_ptr(),
            ..Default::default()
        };
        let descriptor_layout = DescriptorSetLayout::new(device, &descriptor_layout_info);

        let descriptor_pool_info = vk::DescriptorPoolCreateInfo {
            flags: vk::DescriptorPoolCreateFlags::empty(),
            max_sets: max_frames,
            pool_size_count: descriptor_pool_sizes.len() as u32,
            p_pool_sizes: descriptor_pool_sizes.as_ptr(),
            ..Default::default()
        };
        let descriptor_pool = DescriptorPool::new(device, &descriptor_pool_info);

        let set_layouts = [descriptor_layout.handle];

        let descriptor_alloc_info = vk::DescriptorSetAllocateInfo {
            descriptor_pool: descriptor_pool.handle,
            descriptor_set_count: max_frames,
            p_set_layouts: set_layouts.as_ptr(),
            ..Default::default()
        };
        let res = unsafe {
            device
                .ash_device()
                .allocate_descriptor_sets(&descriptor_alloc_info)
        };
        let descriptor_sets = res.unwrap();
        let descriptor_set = descriptor_sets[0];

        let pc_ranges = [];

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo {
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: set_layouts.len() as u32,
            p_set_layouts: set_layouts.as_ptr(),
            push_constant_range_count: pc_ranges.len() as u32,
            p_push_constant_ranges: pc_ranges.as_ptr(),
            ..Default::default()
        };

        let pipeline_layout = PipelineLayout::new(device, &pipeline_layout_info);

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
            module: vertex.module,
            p_name: sources::ENTRY_POINT.as_ptr(),
            p_specialization_info: null(),
            ..Default::default()
        };
        let fragment_stage = vk::PipelineShaderStageCreateInfo {
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            module: fragment.module,
            stage: vk::ShaderStageFlags::FRAGMENT,
            p_name: sources::ENTRY_POINT.as_ptr(),
            p_specialization_info: null(),
            ..Default::default()
        };
        let shader_stages = [vertex_stage, fragment_stage];

        let binding_vertex = vk::VertexInputBindingDescription {
            binding: 0,
            stride: size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        };
        let bindings = [binding_vertex];

        let attribute_xyzw = vk::VertexInputAttributeDescription {
            location: 0,
            binding: binding_vertex.binding,
            format: VERTEX_XYZW::VK_FORMAT,
            offset: 0,
        };
        let attribute_rgba = vk::VertexInputAttributeDescription {
            location: attribute_xyzw.location + 1,
            binding: binding_vertex.binding,
            format: VERTEX_RGBA::VK_FORMAT,
            offset: attribute_xyzw.offset + size_of::<VERTEX_XYZW>() as u32,
        };
        let attribute_uv = vk::VertexInputAttributeDescription {
            location: attribute_rgba.location + 1,
            binding: binding_vertex.binding,
            format: VERTEX_UV::VK_FORMAT,
            offset: attribute_rgba.offset + size_of::<VERTEX_RGBA>() as u32,
        };

        let attributes = [attribute_xyzw, attribute_rgba, attribute_uv];

        let vertex_info = vk::PipelineVertexInputStateCreateInfo {
            flags: vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: bindings.len() as u32,
            p_vertex_binding_descriptions: bindings.as_ptr(),
            vertex_attribute_description_count: attributes.len() as u32,
            p_vertex_attribute_descriptions: attributes.as_ptr(),
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
            layout: pipeline_layout.handle,
            render_pass: render_pass.handle,
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: 0,
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
            pipeline_layout,
            descriptor_set,
            descriptor_pool,
            descriptor_layout,
            fragment,
            vertex,
        }
    }
}

// ================================================================================================================================ //
