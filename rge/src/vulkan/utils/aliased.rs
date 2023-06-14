/*
 *  Crate: RGE
 * Module: Vulkan - Utils - Aliased
 */

//! Utility for creating `Box`ed values that aren't subject to Undefined-Behavior from Pointer-Aliasing.

// ================================================================================================================================ //

use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

// ================================================================================================================================ //

/// A wrapper for `NonNull` that behaves like `Box` but without the `noalias` constraints.
#[repr(transparent)]
pub struct Aliased<T>(NonNull<T>);

// -------------------------------------------------------------------------------------------------------------------------------- //

impl<T> Aliased<T> {
    /// `Box`es the value and wraps it.
    pub fn new(val: T) -> Self {
        let boxed = Box::new(val);
        Self::from_box(boxed)
    }

    /// Consumes a `Box` and allows it to be Aliased.
    pub fn from_box(boxed: Box<T>) -> Self {
        let reference = Box::leak(boxed);
        let nonnull = NonNull::from(reference);
        Self(nonnull)
    }
}

impl<T> Drop for Aliased<T> {
    fn drop(&mut self) {
        let ptr = self.0.as_ptr();
        // Convert back into a `Box` and drop it so the memory is not leaked.
        // SAFETY: The `ptr` is obtained from a `Box` in `Self::from_box`.
        let _boxed = unsafe { Box::from_raw(ptr) };
    }
}

impl<T> Deref for Aliased<T> {
    type Target = NonNull<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Aliased<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ================================================================================================================================ //
