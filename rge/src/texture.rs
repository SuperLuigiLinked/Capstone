/*
 *  Crate: RGE
 * Module: Texture
 */

//! Texture-loading and Manipulation functions.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use core::fmt::{Debug, Display};

use stb_image::image as stbi;

// ================================================================================================================================ //

/// An RGBA Color, represented by 4 8-bit Channels.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct RGBA {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
    /// Alpha component.
    pub a: u8,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl RGBA {
    /// Constructs an RGB Color, using a maxed Alpha channel.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        let a = u8::MAX;
        Self { r, g, b, a }
    }

    /// Constructs an RGBA Color.
    #[allow(clippy::self_named_constructors)]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Constructs an RGBA Color from a 32-Bit 0xAARRGGBB value.
    pub const fn rgba32(value: u32) -> Self {
        let a = (value >> 24) as u8;
        let r = (value >> 16) as u8;
        let g = (value >> 8) as u8;
        let b = value as u8;

        Self { r, g, b, a }
    }

    /// Constructs an RGB Color, using a maxed Alpha channel, converted to 8-Bit Channel Representation.
    pub fn rgb_f(r: f32, g: f32, b: f32) -> Self {
        let r = Self::convert(r);
        let g = Self::convert(g);
        let b = Self::convert(b);
        let a = u8::MAX;
        Self { r, g, b, a }
    }

    /// Constructs an RGBA Color, converted to 8-Bit Channel Representation.
    pub fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r = Self::convert(r);
        let g = Self::convert(g);
        let b = Self::convert(b);
        let a = Self::convert(a);
        Self { r, g, b, a }
    }

    /// Converts from a Normalized [0.0, 1.0] floating-point value to an 8-Bit [0, 255] unsigned-integer value.
    fn convert(val: f32) -> u8 {
        assert!((0.0..=1.0).contains(&val));
        (val * (u8::MAX as f32)) as u8
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

// Debug & Display output in AARRGGBB color-code format.

impl Debug for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}{:02X}{:02X}{:02X}",
            self.a, self.r, self.g, self.b
        )
    }
}

impl Display for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}{:02X}{:02X}{:02X}",
            self.a, self.r, self.g, self.b
        )
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<glsl::vec4> for RGBA {
    fn from(vec: glsl::vec4) -> Self {
        Self::rgba_f(vec.0, vec.1, vec.2, vec.3)
    }
}

impl From<RGBA> for glsl::vec4 {
    fn from(rgba: RGBA) -> Self {
        let r = (rgba.r as glsl::float) / (u8::MAX as glsl::float);
        let g = (rgba.g as glsl::float) / (u8::MAX as glsl::float);
        let b = (rgba.b as glsl::float) / (u8::MAX as glsl::float);
        let a = (rgba.a as glsl::float) / (u8::MAX as glsl::float);

        glsl::vec4(r, g, b, a)
    }
}

// ================================================================================================================================ //

/// A 2-Dimensional Image of 32-Bit RGBA colors, stored in Row-Major order.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Texture {
    /// Width of the `Texture`, in pixels.
    width: usize,

    /// Height of the `Texture`, in pixels.
    height: usize,

    /// `Vec` of Pixel data.
    data: Vec<RGBA>,
}

// ================================================================================================================================ //

impl Texture {
    // ---------------------------------------------------------------- //

    /// Number of Channels in an RGBA color.
    const CHANNEL_NUM: usize = 4;

    // ---------------------------------------------------------------- //

    /// Attempts to load an image from the provided file, optionally chroma keying out a given color.
    pub fn load(path: impl AsRef<std::path::Path>, chroma: Option<RGBA>) -> Result<Self, String> {
        let load_res = stbi::load_with_depth(path, Self::CHANNEL_NUM, false);
        let mut this = Self::stbi_to_rge(load_res)?;

        if let Some(color) = chroma {
            this.chroma(color);
        }
        Ok(this)
    }

    /// Attempts to load an image from the provided buffer, optionally chroma keying out a given color.
    pub fn load_bytes(buffer: &[u8], chroma: Option<RGBA>) -> Result<Self, String> {
        let load_res = stbi::load_from_memory_with_depth(buffer, Self::CHANNEL_NUM, false);
        let mut this = Self::stbi_to_rge(load_res)?;

        if let Some(color) = chroma {
            this.chroma(color);
        }
        Ok(this)
    }

    // ---------------------------------------------------------------- //

    /// Converts from an STBI `Image` to an RGE `Texture`.
    fn stbi_to_rge(load_res: stbi::LoadResult) -> Result<Self, String> {
        match load_res {
            stbi::LoadResult::Error(error) => Err(error),
            stbi::LoadResult::ImageU8(bytes) => Ok(Self::from_stbi_u8(bytes)),
            stbi::LoadResult::ImageF32(floats) => Ok(Self::from_stbi_f32(floats)),
        }
    }

    /// Converts from an STBI `Image` (U8 Format) to an RGE `Texture`.
    fn from_stbi_u8(image: stbi::Image<u8>) -> Self {
        assert_eq!(
            image.depth,
            Self::CHANNEL_NUM,
            "Unsupported Image Format (Non 4-byte)"
        );
        assert_eq!(image.data.len() % Self::CHANNEL_NUM, 0, "Image Parse Error");

        let width = image.width;
        let height = image.height;

        let data = image
            .data
            .chunks_exact(Self::CHANNEL_NUM)
            .map(|chunk| RGBA::rgba(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect();

        Self {
            width,
            height,
            data,
        }
    }

    /// Converts from an STBI `Image` (F32 Format) to an RGE `Texture`.
    fn from_stbi_f32(_image: stbi::Image<f32>) -> Self {
        unimplemented!("Unsupported Image Format (Floating-Point)");
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

impl Texture {
    // ---------------------------------------------------------------- //

    /// Default Color to clear textures with.
    const CLEAR_COLOR: RGBA = RGBA::rgba(0, 0, 0, 0);

    // ---------------------------------------------------------------- //

    /// Creates a new `Texture` with the given dimensions.
    /// # Panics
    /// Panics if `width * height` overflows the limits of a `usize`.
    pub fn new(width: usize, height: usize) -> Self {
        let size = Self::calculate_size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize(size, Self::CLEAR_COLOR);

        Self {
            width,
            height,
            data,
        }
    }

    /// Calculates the Total Size required by the given dimensions.
    /// # Panics
    /// Panics if `width * height` overflows the limits of a `usize`.
    fn calculate_size(width: usize, height: usize) -> usize {
        width.checked_mul(height).expect("Image dimensions too big")
    }

    // ---------------------------------------------------------------- //

    /// Returns the Width of the `Texture`, in pixels.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the Height of the `Texture`, in pixels.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the Total Size of the `Texture`, in pixels.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether or not the Texture has any pixels.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a slice over the pixels in the `Texture`.
    pub fn as_slice(&self) -> &[RGBA] {
        self.data.as_slice()
    }

    /// Returns a mutable slice over the pixels in the `Texture`.
    pub fn as_mut_slice(&mut self) -> &mut [RGBA] {
        self.data.as_mut_slice()
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

// Vulkan-specific functionality.

impl Texture {
    /// Returns the Dimensions of the Texture, in pixels.
    pub(crate) fn vk_extent(&self) -> ash::vk::Extent3D {
        let width = u32::try_from(self.width).unwrap();
        let height = u32::try_from(self.height).unwrap();
        let depth = 1;

        ash::vk::Extent3D {
            width,
            height,
            depth,
        }
    }

    /// Returns the Total Size of the Texture, in bytes.
    pub(crate) fn vk_device_size(&self) -> ash::vk::DeviceSize {
        let width = ash::vk::DeviceSize::try_from(self.width).unwrap();
        let height = ash::vk::DeviceSize::try_from(self.height).unwrap();
        let elems = width.checked_mul(height).unwrap();

        let elem_size = size_of::<RGBA>() as ash::vk::DeviceSize;
        elems.checked_mul(elem_size).unwrap()
    }
}

// ================================================================================================================================ //

// Texture-manipulation functionality.

impl Texture {
    /// Resizes the `Texture` to the given dimensions.
    /// # Panics
    /// Panics if `width * height` overflows the limits of a `usize`.
    pub fn resize(&mut self, width: usize, height: usize) {
        let size = Self::calculate_size(width, height);

        self.data.resize(size, Self::CLEAR_COLOR);
        self.width = width;
        self.height = height;
    }

    /// Clears the `Texture`, setting its dimensions to `0` and clearing the pixel buffer.
    pub fn clear(&mut self) {
        self.width = 0;
        self.height = 0;
        self.data.clear();
    }

    /// Chroma keys out the provided color.
    pub fn chroma(&mut self, color: RGBA) {
        let mut elems = self.data.iter_mut();

        while let Some(elem) = elems.find(move |elem| **elem == color) {
            *elem = Self::CLEAR_COLOR;
        }
    }
}

// ================================================================================================================================ //
