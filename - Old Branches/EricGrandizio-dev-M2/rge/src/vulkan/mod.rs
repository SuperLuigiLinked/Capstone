/*
 *  Crate: RGE
 * Module: Vulkan
 */

//! Utilities for interoperating with Vulkan.
//!
//! Dependencies:
//! * <https://docs.rs/ash/latest/ash/>
//!
//! Documentation:
//! * <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/index.html>

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

mod utils;
pub use utils::*;

mod context;
pub use context::*;

mod renderable;
pub use renderable::*;

mod shaders;
pub use shaders::*;

mod render;
pub use render::*;

// ================================================================================================================================ //

/// All necessary Vulkan State required for GPU-Accelerated Rendering.
pub struct Vulkan {
    /// The Vulkan Renderable, associated with a Window.
    renderable: Option<Aliased<VulkanRenderable>>,

    /// The Vulkan Context.
    context: Aliased<VulkanContext>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Vulkan {
    pub unsafe fn context(&self) -> &VulkanContext {
        self.context.as_ref()
    }

    pub unsafe fn renderable(&self) -> Option<&VulkanRenderable> {
        self.renderable.as_ref().map(|r| r.as_ref())
    }

    pub unsafe fn renderable_mut(&mut self) -> Option<&mut VulkanRenderable> {
        self.renderable.as_mut().map(|r| r.as_mut())
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Vulkan {
    /// Creates a Vulkan Context, deferring Vulkan Renderable creation until a Window is created.
    pub fn new() -> Self {
        let context = VulkanContext::new();
        let renderable = None;

        Self {
            context,
            renderable,
        }
    }

    /// Creates a Vulkan Surface associated with the provided Window.
    /// # Panics
    /// Panics if there is already a Vulkan Surface associated with the Window.
    pub fn create_surface(&mut self, window: WindowHandle, vsync: bool) {
        assert!(self.renderable.is_none());

        let context = unsafe { self.context() };
        let renderable = VulkanRenderable::new(context, window, vsync);
        self.renderable = Some(renderable);
    }

    /// Destroys the Vulkan Surface associated with the provided Window.
    /// # Panics
    /// Panics if there is no Vulkan Surface associated with the Window.
    pub fn destroy_surface(&mut self, window: WindowHandle) {
        assert!(self.renderable.is_some());

        let surface_window = unsafe { self.renderable.as_ref().unwrap().as_ref() }
            .surface
            .window;

        assert!(surface_window == window);

        let _renderable = self.renderable.take();
    }

    /// Updates the current Texture Atlas.
    /// # Panics
    /// Panics if any of the dimensions of the Texture are `0`.
    pub fn update_atlas(&mut self, texture: &Texture) {
        assert!(!texture.is_empty(), "Texture should be of non-zero size");

        // ---------------------------------------------------------------- //

        if let Some(renderable) = self.renderable.as_mut() {
            let vk_window = unsafe { renderable.as_mut() };
            vk_window.atlas = None;

            let atlas = Atlas::new(unsafe { vk_window.device() }, texture);
            vk_window.atlas = Some(atlas);

            let atlas_ref = vk_window.atlas.as_ref().unwrap();
            vk_window.texture_atlas(atlas_ref, texture);
        }
    }
}

// ================================================================================================================================ //
