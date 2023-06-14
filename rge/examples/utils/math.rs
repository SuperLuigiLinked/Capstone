/*
 *  Crate: RGE
 * Module: Tests - Utils - Math
 */

//! Various mathematical functions.

// ================================================================================================================================ //

#![allow(unused)]

use rge::{glsl, rgba, vec4};

// ================================================================================================================================ //

/// Linearly Interpolates between two values.
pub fn lerp(v1: glsl::float, v2: glsl::float, t: glsl::float) -> glsl::float {
    t.mul_add(v2 - v1, v1)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Linearly Interpolates between two colors.
pub fn colerp(v1: glsl::vec4, v2: glsl::vec4, t: glsl::float) -> glsl::vec4 {
    let r = lerp(v1.0, v2.0, t);
    let g = lerp(v1.1, v2.1, t);
    let b = lerp(v1.2, v2.2, t);
    let a = lerp(v1.3, v2.3, t);
    rgba!(r, g, b, a)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Returns a Color on an RGB Color Wheel at `percent` rotation [0.0, 1.0].
pub fn hue(percent: glsl::float) -> glsl::vec4 {
    let norm = percent.fract();
    let phase = (norm * 3.0) as u32 % 3;
    let t = (norm % (1.0 / 3.0)) * 3.0;

    match phase {
        0 => {
            let r = lerp(1.0, 0.0, t);
            let g = lerp(0.0, 1.0, t);
            rgba!(r, g, 0.0)
        }
        1 => {
            let g = lerp(1.0, 0.0, t);
            let b = lerp(0.0, 1.0, t);
            rgba!(0.0, g, b)
        }
        2 => {
            let b = lerp(1.0, 0.0, t);
            let r = lerp(0.0, 1.0, t);
            rgba!(r, 0.0, b)
        }
        _ => unreachable!(),
    }
}

// ================================================================================================================================ //
