/*
 *  Crate: RGE
 * Module: Vulkan - Renderable - Surface
 */

//! Internal utilities for managing Vulkan Window Surfaces.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

/// Wrapper for a `VkSurfaceKHR`.
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html>
pub struct Surface {
    /// Cached size of Window Framebuffer.
    pub size: vk::Extent2D,

    /// Inner `VkSurfaceKHR`.
    pub handle: vk::SurfaceKHR,

    /// OS Handle to Window.
    pub window: WindowHandle,

    /// Pointer to the object responsible for freeing this resource.
    khr_surface_ptr: NonNull<khr::Surface>,
    /// Pointer to the object responsible for creating this resource.
    nat_surface_ptr: NonNull<NativeSurfaceExt>,
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Getter functions for variables behind `Aliased` pointers.
#[allow(clippy::missing_docs_in_private_items)]
#[allow(unused)]
impl Surface {
    pub unsafe fn surface_ext(&self) -> &khr::Surface {
        self.khr_surface_ptr.as_ref()
    }

    pub unsafe fn native_ext(&self) -> &NativeSurfaceExt {
        self.nat_surface_ptr.as_ref()
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[cfg(target_os = "windows")]
impl Surface {
    /// Creates a new Vulkan Surface associated with the given Window.
    pub fn new(context: &VulkanContext, window: WindowHandle) -> Self {
        let exts_ref = unsafe { context.exts() };
        let nat_surface_ptr = NonNull::from(&exts_ref.native_surface);
        let khr_surface_ptr = NonNull::from(&exts_ref.surface);

        let hinstance = unsafe { sys::GetModuleHandleW(null()) } as *const c_void;
        assert!(!hinstance.is_null());

        let hwnd = window as *const c_void;
        assert!(!hwnd.is_null());

        let create_info = vk::Win32SurfaceCreateInfoKHR {
            hinstance,
            hwnd,
            ..Default::default()
        };

        let res = unsafe {
            exts_ref
                .native_surface
                .create_win32_surface(&create_info, None)
        };
        let handle = res.unwrap();

        let size = Self::window_size(window);

        Self {
            size,
            handle,
            window,
            khr_surface_ptr,
            nat_surface_ptr,
        }
    }

    /// Returns the Current (non-cached) size of the Window associated with this Surface.
    pub fn real_size(&self) -> vk::Extent2D {
        Self::window_size(self.window)
    }

    /// Returns the Size of the given Window.
    pub fn window_size(window: WindowHandle) -> vk::Extent2D {
        let mut rect = unsafe { core::mem::zeroed() };
        let res = unsafe { sys::GetClientRect(window, addr_of_mut!(rect)) };
        assert_ne!(res, 0);

        let width = (rect.right - rect.left) as u32;
        let height = (rect.bottom - rect.top) as u32;

        vk::Extent2D { width, height }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { self.surface_ext().destroy_surface(self.handle, None) };
    }
}

// ================================================================================================================================ //
