/*
 *  Crate: RGE
 * Module: Vulkan - Utils - NtString
 */

//! Internal utilities for managing Null-Terminated Strings.

// ================================================================================================================================ //

use core::ffi::c_char;
use core::ffi::CStr;
use core::marker::PhantomData;
use std::borrow::Cow;

// ================================================================================================================================ //

/// Wrapper for a Null-Terminated String, represented by a `const char*`.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct NtString<'a>(*const c_char, PhantomData<&'a c_char>);

// -------------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
impl<'a> NtString<'a> {
    /// Wraps a Byte-String.
    /// # Panics
    /// Panics if there is no Null-Terminator at the End of the Slice.
    pub const fn from_bytes(bytes: &'a [u8]) -> Self {
        let len = bytes.len();
        assert!(len > 0);

        let last = bytes[len - 1];
        assert!(last == 0);

        let ptr = bytes.as_ptr() as *const c_char;
        Self(ptr, PhantomData)
    }

    /// Wraps a Char-Slice.
    /// # Panics
    /// Panics if there is no Null-Terminator at the End of the Slice.
    pub const fn from_chars(bytes: &'a [c_char]) -> Self {
        let len = bytes.len();
        assert!(len > 0);

        let last = bytes[len - 1];
        assert!(last == 0);

        let ptr = bytes.as_ptr();
        Self(ptr, PhantomData)
    }

    /// Wraps a `const char*`.
    /// # Safety
    /// `ptr` must point to a null-terminated buffer that is valid to read from.
    pub const unsafe fn from_ptr(ptr: &'a *const c_char) -> Self {
        Self(*ptr, PhantomData)
    }

    /// Wraps a Char-Slice, up to a certain limit.
    /// # Panics
    /// Panics if there is no Null-Terminator within the Slice.
    pub fn from_chars_cap(bytes: &'a [c_char], cap: usize) -> Self {
        let len = cap.min(bytes.len());

        let has_nt = bytes[0..len].contains(&0);
        assert!(has_nt);

        Self(bytes.as_ptr(), PhantomData)
    }

    // ---------------------------------------------------------------- //

    /// Returns the underlying `const char*`.
    pub const fn as_ptr(&self) -> *const c_char {
        self.0
    }

    /// Returns a `CStr` of the underyling characters.
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }

    /// Returns a Byte-Slice of the underyling characters.
    pub fn as_bytes(&self) -> &[u8] {
        self.as_cstr().to_bytes_with_nul()
    }

    /// Returns a UTF-8 encoded text string from the underlying characters.
    pub fn as_str(&self) -> Cow<str> {
        self.as_cstr().to_string_lossy()
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl<'a> PartialEq for NtString<'a> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.as_cstr();
        let b = other.as_cstr();
        a.eq(b)
    }
}

impl<'a> Eq for NtString<'a> {}

impl<'a> PartialOrd for NtString<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.as_cstr();
        let b = other.as_cstr();
        a.partial_cmp(b)
    }
}

impl<'a> Ord for NtString<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.as_cstr();
        let b = other.as_cstr();
        a.cmp(b)
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl<'a> core::fmt::Debug for NtString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}

impl<'a> core::fmt::Display for NtString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}

// ================================================================================================================================ //

/// Unit-Test for Null-Terminated String functionality.
#[test]
fn nt_string_test() {
    let a = NtString::from_bytes(b"1234\0");
    let b = NtString::from_chars(&[
        '1' as c_char,
        '2' as c_char,
        '3' as c_char,
        '4' as c_char,
        '\0' as c_char,
    ]);
    
    assert_eq!(a, b);
    assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
    assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Equal));

    assert_eq!(format!("{a}"), "1234");
    assert_eq!(format!("{b:?}"), "1234");

    let ptr = a.as_ptr();
    let c = unsafe { NtString::from_ptr(&ptr) };
    assert_eq!(c.as_bytes(), c.clone().as_bytes());
}

// ================================================================================================================================ //
