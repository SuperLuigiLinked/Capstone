/*
 *  Crate: Wyn
 * Module: Cocoa - Types
 */

//! Native types for handling Points, Sizes, and Rectangles, and conversions between Wyn and Native types.

// ================================================================================================================================ //

#![allow(clippy::useless_conversion)]

#[allow(unused_imports)]
use super::*;

pub use crate::common::types::*;

// ================================================================================================================================ //

/// Native type for (x, y) Coordinates.
pub type NativeCoord = sys::CGFloat;

/// Native type for (w, h) Extents.
pub type NativeExtent = sys::CGFloat;

/// Native type for 2-D Points.
pub type NativePoint = sys::NSPoint;

/// Native type for 2-D Sizes.
pub type NativeSize = sys::NSSize;

/// Native type for 2-D Rectangles.
pub type NativeRect = sys::NSRect;

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<NativePoint> for Point {
    fn from(value: NativePoint) -> Self {
        let x = value.x.into();
        let y = value.y.into();
        Self::new(x, y)
    }
}

impl From<Point> for NativePoint {
    fn from(value: Point) -> Self {
        let x = value.x.into();
        let y = value.y.into();
        Self { x, y }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<NativeSize> for Size {
    fn from(value: NativeSize) -> Self {
        let w = value.width.into();
        let h = value.height.into();
        Self::new(w, h)
    }
}

impl From<Size> for NativeSize {
    fn from(value: Size) -> Self {
        let width = value.w.into();
        let height = value.h.into();
        Self { width, height }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<NativeRect> for Rect {
    fn from(value: NativeRect) -> Self {
        let origin = value.origin.into();
        let size = value.size.into();
        Self { origin, size }
    }
}

impl From<Rect> for NativeRect {
    fn from(value: Rect) -> Self {
        let origin = value.origin.into();
        let size = value.size.into();
        Self { origin, size }
    }
}

// ================================================================================================================================ //
