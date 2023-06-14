/*
 *  Crate: Wyn
 * Module: Common - Types
 */

//! Basic types for handling Points, Sizes, and Rectangles.

// ================================================================================================================================ //

/// Type for 2-D Coordinates.
pub type Coord = f64;

/// Type for 2-D Extents.
pub type Extent = f64;

// ================================================================================================================================ //

/// A 2-Dimensional (x, y) point.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Point {
    /// The X-coordinate.
    pub x: Coord,
    /// The Y-coordinate.
    pub y: Coord,
}

impl Point {
    /// Constructs a new Point object.
    pub const fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }
}

// ================================================================================================================================ //

/// A 2-Dimensional (w, h) size.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Size {
    /// The Width.
    pub w: Extent,
    /// The Height.
    pub h: Extent,
}

impl Size {
    /// Constructs a new Size object.
    pub const fn new(w: Extent, h: Extent) -> Self {
        Self { w, h }
    }
}

// ================================================================================================================================ //

/// A 2-Dimensional (x, y, w, h) rectangle.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Rect {
    /// The Point (x, y) of the Top-Left corner.
    pub origin: Point,
    /// The Size (w, h).
    pub size: Size,
}

impl Rect {
    /// Constructs a new Rect object.
    pub const fn new(x: Coord, y: Coord, w: Extent, h: Extent) -> Self {
        Self {
            origin: Point { x, y },
            size: Size { w, h },
        }
    }

    /// Constructs a new Rect object with the given Center and Size.
    pub fn new_centered(center: Point, size: Size) -> Self {
        let ox = (size.w).mul_add(-0.5, center.x);
        let oy = (size.h).mul_add(-0.5, center.y);
        let origin = Point::new(ox, oy);

        Self { origin, size }
    }

    /// Returns the Center-Point of this Rect.
    pub fn center(&self) -> Point {
        let cx = (self.size.w).mul_add(0.5, self.origin.x);
        let cy = (self.size.h).mul_add(0.5, self.origin.y);

        Point::new(cx, cy)
    }

    /// Returns the Aspect-Ratio of this Rect.
    pub fn aspect(&self) -> f64 {
        self.size.w / self.size.h
    }
}

// ================================================================================================================================ //
