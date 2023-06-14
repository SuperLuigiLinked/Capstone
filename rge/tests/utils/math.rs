/*
 *  Crate: RGE
 * Module: Tests - Utils - Math
 */

//! Various mathematical functions.

// ================================================================================================================================ //

#![allow(unused)]

use rge::{glsl, rgba, vec4};

// ================================================================================================================================ //

/// Returns the (x, y) of a 2-Dimension Normalized Rotation in the range (modulo) [0.0, 1.0].
pub fn xy_rotation(angle_pc: glsl::float) -> glsl::vec2 {
    let radians = angle_pc * glsl::float_consts::TAU;

    let (sin, cos) = radians.sin_cos();

    glsl::vec2(cos, sin)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Returns the (x, y, z) of a 3-Dimensional Normalized Rotation in range (modulo) [0.0, 1.0].
pub fn xyz_rotation(
    yaw_pc: glsl::float,
    pitch_pc: glsl::float,
    roll_pc: glsl::float,
) -> glsl::vec3 {
    let glsl::vec2(yc, ys) = self::xy_rotation(yaw_pc);
    let glsl::vec2(pc, ps) = self::xy_rotation(pitch_pc);
    let glsl::vec2(rc, rs) = self::xy_rotation(roll_pc);

    let x = yc * pc - ys * ps * rs;
    let y = yc * ps + ys * pc * rs;
    let z = -ys * rc;

    glsl::vec3(x, y, z)
}

// ================================================================================================================================ //

pub fn oval_angle(half_w: glsl::float, half_h: glsl::float, angle_pc: glsl::float) -> glsl::float {
    let radians = angle_pc * glsl::float_consts::TAU;
    let (sin, cos) = radians.sin_cos();

    let x = cos * half_w;
    let y = sin * half_h;

    y.atan2(x)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Returns the Distance of the Point on the Perimeter of an Oval at a Normalized Angle in [0.0, 1.0].
pub fn oval_radius(half_w: glsl::float, half_h: glsl::float, angle_pc: glsl::float) -> glsl::float {
    let radians = angle_pc * glsl::float_consts::TAU;
    let (sin, cos) = radians.sin_cos();

    let w2 = half_w * half_w;
    let h2 = half_h * half_h;
    let c2 = cos * cos;
    let s2 = sin * sin;

    ((w2 * h2) / ((h2 * c2) + (w2 * s2))).sqrt()
}

// -------------------------------------------------------------------------------------------------------------------------------- //

pub fn oval_point(
    origin: glsl::vec2,
    half_w: glsl::float,
    half_h: glsl::float,
    angle_pc: glsl::float,
) -> glsl::vec2 {
    let radians = angle_pc * glsl::float_consts::TAU;
    let (sin, cos) = radians.sin_cos();

    let radius = self::oval_radius(half_w, half_h, angle_pc);

    let x = cos.mul_add(radius, origin.0);
    let y = sin.mul_add(radius, origin.1);
    glsl::vec2(x, y)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

pub fn circle_point(
    origin: glsl::vec2,
    half_w: glsl::float,
    half_h: glsl::float,
    angle_pc: glsl::float,
) -> glsl::vec2 {
    let radians = angle_pc * glsl::float_consts::TAU;
    let (sin, cos) = radians.sin_cos();

    let x = cos.mul_add(half_w, origin.0);
    let y = sin.mul_add(half_h, origin.1);
    glsl::vec2(x, y)
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Rotates a P: f32oint about a Center-Point by an angle in Radians.
pub fn rotate_about(point: glsl::vec2, center: glsl::vec2, angle_pc: glsl::float) -> glsl::vec2 {
    let radians = angle_pc * glsl::float_consts::TAU;

    let relative_x = point.0 - center.0;
    let relative_y = point.1 - center.1;
    let relative_distance = (relative_y).hypot(relative_x);
    let relative_radians = (relative_y).atan2(relative_x);

    let new_radians = relative_radians + radians;
    let (sin, cos) = new_radians.sin_cos();

    let new_x = relative_distance.mul_add(cos, center.0);
    let new_y = relative_distance.mul_add(sin, center.1);
    glsl::vec2(new_x, new_y)
}

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
