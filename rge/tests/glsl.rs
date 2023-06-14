/*
 *  Crate: RGE
 *   Test: GLSL
 */

//! This test tests various GLSL-related operations.

// ================================================================================================================================ //

mod utils;

// ================================================================================================================================ //

#[test]
pub fn image() {
    utils::timeout::test_deadline(5.0);

    test_main();
}

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

#[allow(unused_imports)]
use rge::{glsl, rgba, vec2, vec3, vec4, Vertex, VertexUV};

// ================================================================================================================================ //

fn test_main() {
    // ---------------------------------------------------------------- //

    assert_eq!(vec2!(), glsl::vec2(0.0, 0.0));
    assert_eq!(vec2!(1.0), glsl::vec2(1.0, 0.0));
    assert_eq!(vec2!(1.0, 1.0), glsl::vec2(1.0, 1.0));

    assert_eq!(vec3!(), glsl::vec3(0.0, 0.0, 0.0));
    assert_eq!(vec3!(1.0), glsl::vec3(1.0, 0.0, 0.0));
    assert_eq!(vec3!(1.0, 1.0), glsl::vec3(1.0, 1.0, 0.0));
    assert_eq!(vec3!(1.0, 1.0, 1.0), glsl::vec3(1.0, 1.0, 1.0));

    assert_eq!(vec4!(), glsl::vec4(0.0, 0.0, 0.0, 0.0));
    assert_eq!(vec4!(1.0), glsl::vec4(1.0, 0.0, 0.0, 0.0));
    assert_eq!(vec4!(1.0, 1.0), glsl::vec4(1.0, 1.0, 0.0, 0.0));
    assert_eq!(vec4!(1.0, 1.0, 1.0), glsl::vec4(1.0, 1.0, 1.0, 0.0));
    assert_eq!(vec4!(1.0, 1.0, 1.0, 1.0), glsl::vec4(1.0, 1.0, 1.0, 1.0));

    assert_eq!(rgba!(), glsl::vec4(0.0, 0.0, 0.0, 0.0));
    assert_eq!(rgba!(0.5), glsl::vec4(0.5, 0.5, 0.5, 1.0));
    assert_eq!(rgba!(0.5, 0.5), glsl::vec4(0.5, 0.5, 0.5, 0.5));
    assert_eq!(rgba!(0.5, 0.5, 0.5), glsl::vec4(0.5, 0.5, 0.5, 1.0));
    assert_eq!(rgba!(0.5, 0.5, 0.5, 0.5), glsl::vec4(0.5, 0.5, 0.5, 0.5));

    // ---------------------------------------------------------------- //

    let vec = glsl::ivec2(1, 2);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::ivec2::default()).is_empty());

    let vec = glsl::ivec3(1, 2, 3);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::ivec3::default()).is_empty());

    let vec = glsl::ivec4(1, 2, 3, 4);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::ivec4::default()).is_empty());

    // ---------------------------------------------------------------- //

    let vec = glsl::uvec2(1, 2);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::uvec2::default()).is_empty());

    let vec = glsl::uvec3(1, 2, 3);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::uvec3::default()).is_empty());

    let vec = glsl::uvec4(1, 2, 3, 4);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::uvec4::default()).is_empty());

    // ---------------------------------------------------------------- //

    let vec = glsl::vec2(1.0, 2.0);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::vec2::default()).is_empty());

    let vec = glsl::vec3(1.0, 2.0, 3.0);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::vec3::default()).is_empty());

    let vec = glsl::vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::vec4::default()).is_empty());

    // ---------------------------------------------------------------- //

    let vec = glsl::dvec2(1.0, 2.0);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::dvec2::default()).is_empty());

    let vec = glsl::dvec3(1.0, 2.0, 3.0);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::dvec3::default()).is_empty());

    let vec = glsl::dvec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(vec, vec.clone());
    assert!(!format!("{:?}", glsl::dvec4::default()).is_empty());

    // ---------------------------------------------------------------- //

    let vertex = Vertex {
        xyzw: glsl::vec4(0.0, 0.0, 0.0, 0.0),
        rgba: glsl::vec4(0.0, 0.0, 0.0, 0.0),
    };

    assert_eq!(vertex.clone(), Vertex::default());

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //
