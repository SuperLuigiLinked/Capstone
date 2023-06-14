/*
 *  Crate: Wyn
 * Module: Common - Nonzero
 */

//! Provides a Generic Type Alias for NonZero types.
//!
//! This is useful for when you want to use the NonZero variant of a Type behind a Type Alias.
//! You would not be able to hard-code the proper representation, hence the need for Generics to select the right type.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

pub use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
pub use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
pub use core::ptr::NonNull;

// ================================================================================================================================ //

/// Represents types with a NonZero variant.
pub trait NonZeroType {
    /// The underlying type representation.
    type Repr;
}

/// Generic type alias for NonZero types.
pub type NonZero<T> = <T as NonZeroType>::Repr;

// -------------------------------------------------------------------------------------------------------------------------------- //

impl NonZeroType for u8 {
    type Repr = NonZeroU8;
}

impl NonZeroType for u16 {
    type Repr = NonZeroU16;
}

impl NonZeroType for u32 {
    type Repr = NonZeroU32;
}

impl NonZeroType for u64 {
    type Repr = NonZeroU64;
}

impl NonZeroType for usize {
    type Repr = NonZeroUsize;
}

impl NonZeroType for i8 {
    type Repr = NonZeroI8;
}

impl NonZeroType for i16 {
    type Repr = NonZeroI16;
}

impl NonZeroType for i32 {
    type Repr = NonZeroI32;
}

impl NonZeroType for i64 {
    type Repr = NonZeroI64;
}

impl NonZeroType for isize {
    type Repr = NonZeroIsize;
}

// ================================================================================================================================ //
