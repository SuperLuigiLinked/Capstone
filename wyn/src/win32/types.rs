/*
 *  Crate: Wyn
 * Module: Win32 - Types
 */

//! Native types for handling Points, Sizes, and Rectangles, and conversions between Wyn and Native types.

// ================================================================================================================================ //

#![allow(clippy::useless_conversion)]

#[allow(unused_imports)]
use super::*;

pub use crate::common::types::*;

// ================================================================================================================================ //

/// Native type for (x, y) Coordinates.
pub type NativeCoord = sys::LONG;

/// Native type for (w, h) Extents.
pub type NativeExtent = sys::LONG;

/// Native type for 2-D Points.
pub type NativePoint = sys::POINT;

/// Native type for 2-D Sizes.
pub type NativeSize = sys::SIZE;

/// Native type for 2-D Rectangles.
pub type NativeRect = sys::RECT;

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<NativePoint> for Point {
    fn from(value: NativePoint) -> Self {
        let x = value.x as _;
        let y = value.y as _;
        Self { x, y }
    }
}

impl From<Point> for NativePoint {
    fn from(value: Point) -> Self {
        let x = value.x as _;
        let y = value.y as _;
        Self { x, y }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<NativeSize> for Size {
    fn from(value: NativeSize) -> Self {
        let w = value.cx as _;
        let h = value.cy as _;
        Self { w, h }
    }
}

impl From<Size> for NativeSize {
    fn from(value: Size) -> Self {
        let cx = value.w as _;
        let cy = value.h as _;
        Self { cx, cy }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<NativeRect> for Rect {
    fn from(value: NativeRect) -> Self {
        let x = value.left as _;
        let y = value.top as _;
        let w = (value.right - value.left) as _;
        let h = (value.bottom - value.top) as _;
        let origin = Point { x, y };
        let size = Size { w, h };
        Self { origin, size }
    }
}

impl From<Rect> for NativeRect {
    fn from(value: Rect) -> Self {
        let left = value.origin.x as _;
        let top = value.origin.y as _;
        let right = (value.origin.x + value.size.w) as _;
        let bottom = (value.origin.y + value.size.h) as _;
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

// ================================================================================================================================ //
