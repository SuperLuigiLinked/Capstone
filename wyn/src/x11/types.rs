/*
 *  Crate: Wyn
 * Module: X11 - Types
 */

//! Native types for handling Points, Sizes, and Rectangles, and conversions between Wyn and Native types.

// ================================================================================================================================ //

#![allow(clippy::useless_conversion)]

#[allow(unused_imports)]
use super::*;

pub use crate::common::types::*;

// ================================================================================================================================ //

/// Native type for (x, y) Coordinates.
pub type NativeCoord = i16;

/// Native type for (w, h) Extents.
pub type NativeExtent = u16;

/// Native type for 2-D Points.
pub type NativePoint = sys::xcb_point_t;

/// Native type for 2-D Sizes.
pub type NativeSize = sys::xcb_point_t;

/// Native type for 2-D Rectangles.
pub type NativeRect = sys::xcb_rectangle_t;

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
        let w = value.x as _;
        let h = value.y as _;
        Self { w, h }
    }
}

impl From<Size> for NativeSize {
    fn from(value: Size) -> Self {
        let x = value.w as _;
        let y = value.h as _;
        Self { x, y }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl From<NativeRect> for Rect {
    fn from(value: NativeRect) -> Self {
        let x = value.x as _;
        let y = value.y as _;
        let w = value.width as _;
        let h = value.height as _;
        let origin = Point { x, y };
        let size = Size { w, h };
        Self { origin, size }
    }
}

impl From<Rect> for NativeRect {
    fn from(value: Rect) -> Self {
        let x = value.origin.x as _;
        let y = value.origin.y as _;
        let width = value.size.w as _;
        let height = value.size.h as _;
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

// ================================================================================================================================ //
