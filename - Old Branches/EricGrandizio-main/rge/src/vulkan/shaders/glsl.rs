/*
 *  Crate: RGE
 * Module: Vulkan - Shaders - GLSL
 */

//! Rust-wrappers around GLSL Data Types.
//!
//! <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)>

// ================================================================================================================================ //

#![allow(non_camel_case_types)]
#![allow(unused)]

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use core::ops::{Deref, DerefMut};

pub use core::f32::consts as float_consts;
pub use core::f64::consts as double_consts;

// ================================================================================================================================ //

/// Internal Trait for associating Rust Types with Vulkan Image Formats.
pub(crate) trait VulkanFormat {
    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormat.html>
    const VK_FORMAT: vk::Format;
}

// ================================================================================================================================ //

// Scalars

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Scalars>
pub type int = i32;

impl VulkanFormat for int {
    const VK_FORMAT: vk::Format = vk::Format::R32_SINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Scalars>
pub type uint = u32;

impl VulkanFormat for uint {
    const VK_FORMAT: vk::Format = vk::Format::R32_UINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Scalars>
pub type float = f32;

impl VulkanFormat for float {
    const VK_FORMAT: vk::Format = vk::Format::R32_SFLOAT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Scalars>
pub type double = f64;

impl VulkanFormat for double {
    const VK_FORMAT: vk::Format = vk::Format::R64_SFLOAT;
}

// ================================================================================================================================ //

// Vectors

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct ivec2(pub int, pub int);

impl VulkanFormat for ivec2 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32_SINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct ivec3(pub int, pub int, pub int);

impl VulkanFormat for ivec3 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32B32_SINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct ivec4(pub int, pub int, pub int, pub int);

impl VulkanFormat for ivec4 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32B32A32_SINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct uvec2(pub uint, pub uint);

impl VulkanFormat for uvec2 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32_UINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct uvec3(pub uint, pub uint, pub uint);

impl VulkanFormat for uvec3 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32B32_UINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct uvec4(pub uint, pub uint, pub uint, pub uint);

impl VulkanFormat for uvec4 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32B32A32_UINT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct vec2(pub float, pub float);

impl VulkanFormat for vec2 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32_SFLOAT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct vec3(pub float, pub float, pub float);

impl VulkanFormat for vec3 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32B32_SFLOAT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct vec4(pub float, pub float, pub float, pub float);

impl VulkanFormat for vec4 {
    const VK_FORMAT: vk::Format = vk::Format::R32G32B32A32_SFLOAT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct dvec2(pub double, pub double);

impl VulkanFormat for dvec2 {
    const VK_FORMAT: vk::Format = vk::Format::R64G64_SFLOAT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct dvec3(pub double, pub double, pub double);

impl VulkanFormat for dvec3 {
    const VK_FORMAT: vk::Format = vk::Format::R64G64B64_SFLOAT;
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)#Vectors>
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct dvec4(pub double, pub double, pub double, pub double);

impl VulkanFormat for dvec4 {
    const VK_FORMAT: vk::Format = vk::Format::R64G64B64A64_SFLOAT;
}

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

// Structs

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Constants Pushed into the Vulkan Pipeline.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct PushConstants {
    /// [CURRENTLY UNUSED] Internal debug constant.
    pub rotate_pc: glsl::float,
}

impl PushConstants {
    /// Returns a view over the bytes of this set of Push Constants.
    pub const fn as_bytes(&self) -> &[u8] {
        let data = self as *const Self as *const u8;
        let len = size_of::<Self>();
        unsafe { core::slice::from_raw_parts(data, len) }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// A Vertex, used to represent points/colors on various shapes.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Vertex {
    /// Position of the Vertex. Only XY values are used for positioning. Z values represent Depth/Layer [TODO]. W values represent Point-Size.
    pub xyzw: glsl::vec4,

    /// Color of the Vertex.
    pub rgba: glsl::vec4,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Creates a `vec2` using 0-2 arguments. Unspecified values default to `0.0`.
#[macro_export]
macro_rules! vec2 {
    () => {
        $crate::glsl::vec2(0.0, 0.0)
    };
    ($x:expr) => {
        $crate::glsl::vec2($x, 0.0)
    };
    ($x:expr, $y:expr) => {
        $crate::glsl::vec2($x, $y)
    };
}

/// Creates a `vec3` using 0-3 arguments. Unspecified values default to `0.0`.
#[macro_export]
macro_rules! vec3 {
    () => {
        $crate::glsl::vec3(0.0, 0.0, 0.0)
    };
    ($x:expr) => {
        $crate::glsl::vec3($x, 0.0, 0.0)
    };
    ($x:expr, $y:expr) => {
        $crate::glsl::vec3($x, $y, 0.0)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::glsl::vec3($x, $y, $z)
    };
}

/// Creates a `vec4` using 0-4 arguments. Unspecified values default to `0.0`.
#[macro_export]
macro_rules! vec4 {
    () => {
        $crate::glsl::vec4(0.0, 0.0, 0.0, 0.0)
    };
    ($x:expr) => {
        $crate::glsl::vec4($x, 0.0, 0.0, 0.0)
    };
    ($x:expr, $y:expr) => {
        $crate::glsl::vec4($x, $y, 0.0, 0.0)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::glsl::vec4($x, $y, $z, 0.0)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::glsl::vec4($x, $y, $z, $w)
    };
}

/// Creates a `vec4` using 0, 1, 2, 3, or 4 arguments.
/// 0 Arguments: Unspecified RGB values default to `0.0`. Alpha value defaults to `0.0`.
/// 1 Argument : First Argument is used for all RGB values. Alpha value defaults to `1.0`.
/// 2 Arguments: First Argument is used for all RGB values. Second Argument is used for Alpha value.
/// 3 Arguments: Specified RGB values are used. Alpha value defaults to `1.0`.
/// 4 Arguments: Specified RGB and Alpha values are used.
#[macro_export]
macro_rules! rgba {
    () => {
        $crate::glsl::vec4(0.0, 0.0, 0.0, 0.0)
    };
    ($v:expr) => {
        $crate::glsl::vec4($v, $v, $v, 1.0)
    };
    ($v:expr, $a:expr) => {
        $crate::glsl::vec4($v, $v, $v, $a)
    };
    ($r:expr, $g:expr, $b:expr) => {
        $crate::glsl::vec4($r, $g, $b, 1.0)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        $crate::glsl::vec4($r, $g, $b, $a)
    };
}

// ================================================================================================================================ //
