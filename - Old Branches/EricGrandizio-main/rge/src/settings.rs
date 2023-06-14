/*
 *  Crate: RGE
 * Module: Settings
 */

//! `GameEngine`-related settings that are configurable by Users at runtime.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

pub use vulkan::glsl;

// ================================================================================================================================ //

/// A list of Settings to Initialize a `GameEngine`.
#[derive(Clone, Copy, Debug)]
pub struct GameEngineSettings {
    /// Frames-Per-Second of the Application.
    pub fps: f64,

    /// VSYNC toggle (On: No tearing | Off: May tear).
    pub vsync: bool,

    /// Initial Width of the Window.
    pub width: f64,

    /// Initial Height o the Window.
    pub height: f64,
}

// ================================================================================================================================ //

/// A list of Settings to control a `Window`.
#[derive(Clone, Default, Debug)]
pub struct WindowSettings {
    /// The name of the `Window`.
    pub name: String,
}

// ================================================================================================================================ //

/// A list off Settings to control the Renderer.
#[derive(Clone, Default, Debug)]
pub struct RenderSettings {
    // -------------------------------- //
    /// Background fill color.
    pub backcolor: glsl::vec4,

    /// A list of Points, by Vertices.
    pub(crate) point_list: Vec<glsl::Vertex>,

    /// A list of Lines, by Vertices.
    pub(crate) line_list: Vec<glsl::Vertex>,

    /// A list of Triangles, by Vertices.
    pub(crate) tri_list: Vec<glsl::Vertex>,

    /// A list of Line Strips, by Vertices.
    pub(crate) line_strip: Vec<glsl::Vertex>,

    /// A list of Line Strips, by Indices.
    pub(crate) line_strip_idx: Vec<u16>,

    /// A list of Triangle Strips, by Vertices.
    pub(crate) tri_strip: Vec<glsl::Vertex>,

    /// A list of Triangle Strips, by Indices.
    pub(crate) tri_strip_idx: Vec<u16>,

    /// A list of Triangle Fans, by Vertices.
    pub(crate) tri_fan: Vec<glsl::Vertex>,

    /// A list of Triangle Fans, by Indices.
    pub(crate) tri_fan_idx: Vec<u16>,

    // -------------------------------- //
    /// Internal update count.
    pub(crate) updates: usize,

    /// Internal render count.
    pub(crate) renders: usize,

    /// Internal vsync state.
    pub(crate) vsync: bool,

    /// Internal fps state.
    pub(crate) fps: f64,
    // -------------------------------- //
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl RenderSettings {
    /// Special Index value to indicate the end of a Strip/Fan.
    const SENTINEL_IDX: u16 = 0xFFFF;

    /// Clears the render settings to the default settings.
    pub fn clear(&mut self) {
        self.backcolor = glsl::vec4(0.0, 0.0, 0.0, 1.0);

        self.point_list.clear();
        self.line_list.clear();
        self.tri_list.clear();

        self.line_strip.clear();
        self.line_strip_idx.clear();

        self.tri_strip.clear();
        self.tri_strip_idx.clear();

        self.tri_fan.clear();
        self.tri_fan_idx.clear();
    }

    /// Adds a Point to the render batch.
    pub fn point(&mut self, vertex: glsl::Vertex) {
        self.point_list.push(vertex);
    }

    /// Adds a Line to the render batch.
    pub fn line(&mut self, vertices: &[glsl::Vertex; 2]) {
        for vertex in vertices {
            self.line_list.push(*vertex);
        }
    }

    /// Adds a Triangle to the render batch.
    pub fn triangle(&mut self, vertices: &[glsl::Vertex; 3]) {
        for vertex in vertices {
            self.tri_list.push(*vertex);
        }
    }

    /// Adds a Line Strip to the render batch.
    pub fn line_strip(&mut self, vertices: &[glsl::Vertex]) {
        assert!(
            vertices.len() >= 2,
            "Line-Strips must contain 2 or more Vertices."
        );

        let mut idx = self.line_strip.len() as u16;
        for vertex in vertices {
            self.line_strip.push(*vertex);
            self.line_strip_idx.push(idx);
            idx += 1;
        }
        self.line_strip_idx.push(Self::SENTINEL_IDX);
    }

    /// Adds a Triangle Strip to the render batch.
    pub fn triangle_strip(&mut self, vertices: &[glsl::Vertex]) {
        assert!(
            vertices.len() >= 3,
            "Triangle-Strips must contain 3 or more Vertices."
        );

        let mut idx = self.tri_strip.len() as u16;
        for vertex in vertices {
            self.tri_strip.push(*vertex);
            self.tri_strip_idx.push(idx);
            idx += 1;
        }
        self.tri_strip_idx.push(Self::SENTINEL_IDX);
    }

    /// Adds a Triangle Fan to the render batch.
    pub fn triangle_fan(&mut self, vertices: &[glsl::Vertex]) {
        assert!(
            vertices.len() >= 3,
            "Triangle-Fans must contain 3 or more Vertices."
        );

        let mut idx = self.tri_fan.len() as u16;
        for vertex in vertices {
            self.tri_fan.push(*vertex);
            self.tri_fan_idx.push(idx);
            idx += 1;
        }
        self.tri_fan_idx.push(Self::SENTINEL_IDX);
    }
}

// ================================================================================================================================ //
