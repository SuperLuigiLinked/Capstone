/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - Sources
 */

//! Internal utilities for retrieving Shader source information.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// A source filename and its source code.
pub struct ShaderSource {
    /// Filename.
    pub name: &'static str,

    /// Source Code.
    pub text: &'static str,

    /// Shader Type.
    pub kind: shaderc::ShaderKind,
}

/// Creates a `ShaderSource` constant.
macro_rules! shader_source {
    ($name:literal, $kind:expr) => {
        ShaderSource {
            name: $name,
            text: include_str!($name),
            kind: $kind,
        }
    };
}

// ================================================================================================================================ //

/// Name of Entry-Point Function in Shader Code.
pub const ENTRY_POINT: NtString = NtString::from_bytes(b"main\0");

// -------------------------------------------------------------------------------------------------------------------------------- //

/// RGBA-Shader sources.
pub mod rgba {
    use super::*;

    pub use super::ENTRY_POINT;

    /// Vertex Shader.
    pub const VERTEX: ShaderSource =
        shader_source!("glsl_shaders/rgba/shader.vert", shaderc::ShaderKind::Vertex);

    /// Fragment Shader.
    pub const FRAGMENT: ShaderSource = shader_source!(
        "glsl_shaders/rgba/shader.frag",
        shaderc::ShaderKind::Fragment
    );
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// UV-Shader sources.
pub mod uv {
    use super::*;

    pub use super::ENTRY_POINT;

    /// Vertex Shader.
    pub const VERTEX: ShaderSource =
        shader_source!("glsl_shaders/uv/shader.vert", shaderc::ShaderKind::Vertex);

    /// Fragment Shader.
    pub const FRAGMENT: ShaderSource =
        shader_source!("glsl_shaders/uv/shader.frag", shaderc::ShaderKind::Fragment);
}

// ================================================================================================================================ //
