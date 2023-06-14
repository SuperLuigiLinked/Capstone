/*
 *  Crate: RGE
 * Module: Vulkan - Shaders
 */

//! Internal utilities for managing Shader-State in Vulkan.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

mod shader_module;
pub use shader_module::*;

mod render_pass;
pub use render_pass::*;

mod descriptors;
pub use descriptors::*;

mod pipeline_layout;
pub use pipeline_layout::*;

mod graphics_pipelines;
pub use graphics_pipelines::*;

mod command_pool;
pub use command_pool::*;

mod memory;
pub use memory::*;

mod buffers;
pub use buffers::*;

mod atlas;
pub use atlas::*;

pub mod glsl;

pub mod sources;

mod glsl_shaders;
pub use glsl_shaders::*;

// ================================================================================================================================ //
