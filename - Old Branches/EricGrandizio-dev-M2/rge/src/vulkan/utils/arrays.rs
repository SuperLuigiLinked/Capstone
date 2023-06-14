/*
 *  Crate: RGE
 * Module: Vulkan - Utils - Arrays
 */

//! Internal utilities for managing Arrays and Slices.

// ================================================================================================================================ //

use core::mem::MaybeUninit;

// ================================================================================================================================ //

/// Converts a Slice of any type to a Slice over its bytes.
#[inline]
pub const fn bytes_ref<T>(slice: &[T]) -> &[u8] {
    let len = core::mem::size_of::<T>() * slice.len();
    let data = slice.as_ptr() as *const u8;
    unsafe { core::slice::from_raw_parts(data, len) }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Concatenates two arrays.
/// # Panics
/// Panics if the Output Array's length is not equal to the sum of the Input Arrays' lengths.
#[allow(unused)]
pub fn array_concat<T: Sized + Copy, const A: usize, const B: usize, const C: usize>(
    a: [T; A],
    b: [T; B],
) -> [T; C] {
    assert_eq!(A + B, C);

    // The snippet bellow is taken from the Rust Standard Library (`MaybeUninit::uninit_array`).
    // It is currently unstable, so it cannot be used directly.

    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    let mut uninit_c = unsafe { MaybeUninit::<[MaybeUninit<T>; C]>::uninit().assume_init() };

    let uninit_a: &[MaybeUninit<T>; A] = unsafe { core::mem::transmute(&a) };
    let uninit_b: &[MaybeUninit<T>; B] = unsafe { core::mem::transmute(&b) };

    uninit_c[0..A].copy_from_slice(uninit_a);
    uninit_c[A..C].copy_from_slice(uninit_b);

    unsafe { core::mem::transmute_copy(&uninit_c) }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Intersperses two arrays of equal size.
/// # Panics
/// Panics if the Output Array's length is not equal to the sum of the Input Arrays' lengths.
#[allow(unused)]
pub fn array_intersperse<T: Sized + Copy, const IN: usize, const OUT: usize>(
    a: [T; IN],
    b: [T; IN],
) -> [T; OUT] {
    assert_eq!(IN + IN, OUT);

    // The snippet bellow is taken from the Rust Standard Library (`MaybeUninit::uninit_array`).
    // It is currently unstable, so it cannot be used directly.

    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    let mut uninit_c = unsafe { MaybeUninit::<[MaybeUninit<T>; OUT]>::uninit().assume_init() };

    let uninit_a: &[MaybeUninit<T>; IN] = unsafe { core::mem::transmute(&a) };
    let uninit_b: &[MaybeUninit<T>; IN] = unsafe { core::mem::transmute(&b) };

    for i in 0..IN {
        uninit_c[2 * i] = uninit_a[i];
        uninit_c[2 * i + 1] = uninit_b[i];
    }

    unsafe { core::mem::transmute_copy(&uninit_c) }
}

// ================================================================================================================================ //

/// Unit-Test to ensure Array-Operations work successfully.
#[test]
fn array_ops_test() {
    {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let c: [_; 8] = array_concat(a, b);
        assert_eq!(c, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
    {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let c: [_; 8] = array_intersperse(a, b);
        assert_eq!(c, [1, 5, 2, 6, 3, 7, 4, 8]);
    }
}

// ================================================================================================================================ //
