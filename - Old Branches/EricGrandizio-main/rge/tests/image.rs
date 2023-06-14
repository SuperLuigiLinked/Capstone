/*
 *  Crate: RGE
 *   Test: Image
 */

//! This test tests Texture, Image-Loading, and RGBA color functionality.
//!
//! First, it tests all the RGBA-related functionality.
//!
//! Then, it loads an Image from a file at run-time.
//! The same Image is also embeded in the program's memory.
//!
//! It will then proceed to manipulate the Textures, compare them, etc...
//!
//! If the images load successfully and the contents are as expected, then the test passes.

// ================================================================================================================================ //

mod utils;

// ================================================================================================================================ //

#[test]
pub fn image() {
    utils::timeout::test_deadline(5.0);

    test_main();
}

// ================================================================================================================================ //

use rge::{Texture, RGBA};

#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4};

// ================================================================================================================================ //

/// The File, pre-loaded at Compile-Time.
const IMAGE_BYTES: &[u8; 124] = include_bytes!("images/RGB.png");

// ================================================================================================================================ //

fn test_main() {
    // ---------------------------------------------------------------- //

    let c1 = RGBA::rgb(1, 2, 3);
    assert_eq!(c1.r, 1);
    assert_eq!(c1.g, 2);
    assert_eq!(c1.b, 3);
    assert_eq!(c1.a, 0xFF);

    let c2 = RGBA::rgba(1, 2, 3, 4);
    assert_eq!(c2.r, 1);
    assert_eq!(c2.g, 2);
    assert_eq!(c2.b, 3);
    assert_eq!(c2.a, 4);

    let c3 = RGBA::rgb_f(0.0, 0.5, 1.0);
    assert_eq!(c3.r, 0x00);
    assert_eq!(c3.g, 0x7F);
    assert_eq!(c3.b, 0xFF);
    assert_eq!(c3.a, 0xFF);

    let c4 = RGBA::rgba_f(1.0, 0.5, 0.0, 0.5);
    assert_eq!(c4.r, 0xFF);
    assert_eq!(c4.g, 0x7F);
    assert_eq!(c4.b, 0x00);
    assert_eq!(c4.a, 0x7F);

    // ---------------------------------------------------------------- //

    let color = RGBA {
        a: 0xDE,
        r: 0xAD,
        g: 0xBE,
        b: 0xEF,
    };
    assert_eq!(color.to_string(), "DEADBEEF");

    let s1 = format!("{color:?}");
    let s2 = format!("{color}");

    assert_eq!(s1, s2);

    // ---------------------------------------------------------------- //

    let rel_path_a = std::path::Path::new("images/RGB.png");
    let rel_path_b = std::path::Path::new("tests/images/RGB.png");
    let rel_path_c = std::path::Path::new("rge/tests/images/RGB.png");

    let path = if rel_path_a.exists() {
        rel_path_a
    } else if rel_path_b.exists() {
        rel_path_b
    } else if rel_path_c.exists() {
        rel_path_c
    } else {
        panic!("Cannot find image file!");
    };

    // ---------------------------------------------------------------- //

    let res = Texture::load(path, None);
    let file_image = match res {
        Ok(image) => image,
        Err(msg) => panic!("Run-Time Image was unable to be loaded: \"{msg}\""),
    };

    assert_eq!(file_image.width(), 2);
    assert_eq!(file_image.height(), 2);
    assert_eq!(file_image.len(), 4);

    // ---------------------------------------------------------------- //

    let file_pixels = file_image.as_slice();
    assert_eq!(file_pixels[0], RGBA::rgba(0x00, 0x00, 0x00, 0x00));
    assert_eq!(file_pixels[1], RGBA::rgba(0xFF, 0x00, 0x00, 0xFF));
    assert_eq!(file_pixels[2], RGBA::rgba(0x00, 0xFF, 0x00, 0xFF));
    assert_eq!(file_pixels[3], RGBA::rgba(0x00, 0x00, 0xFF, 0xFF));

    // ---------------------------------------------------------------- //

    let res = Texture::load_bytes(IMAGE_BYTES, None);
    let mut memory_image = match res {
        Ok(image) => image,
        Err(msg) => panic!("Compile-Time Image was unable to be loaded: \"{msg}\""),
    };

    assert_eq!(file_image, memory_image);

    // ---------------------------------------------------------------- //

    let res = Texture::load_bytes(IMAGE_BYTES, Some(RGBA::rgb(0xFF, 0x00, 0x00)));
    let chroma_image = match res {
        Ok(image) => image,
        Err(msg) => panic!("Compile-Time Image was unable to be loaded: \"{msg}\""),
    };

    memory_image.as_mut_slice()[1] = RGBA::default();
    assert_eq!(chroma_image, memory_image);

    // ---------------------------------------------------------------- //

    let mut new_texture = Texture::new(0, 0);
    assert!(new_texture.is_empty());

    new_texture.resize(2, 2);
    assert!(!new_texture.is_empty());

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //
