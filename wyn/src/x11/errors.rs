/*
 *  Crate: Wyn
 * Module: X11 - Errors
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use std::error::Error;

// ================================================================================================================================ //

/// Native error-code for XCB Connection Errors.
pub(crate) type NativeXcbConnectionCode = ::std::os::raw::c_int;

/// Nonzero-type for native XCB Connection Errors.
pub(crate) type NonzeroXcbConnectionCode = NonZero<NativeXcbConnectionCode>;

/// A wrapper for XCB Connection Errors.
#[repr(transparent)]
pub struct XcbConnectionError(NonzeroXcbConnectionCode);

/// A result for XCB Connections.
pub type XcbConnectionResult<T> = Result<T, XcbConnectionError>;

impl XcbConnectionError {
    /// Attempts to construct a new `XcbConnectionError`.
    pub(crate) fn new(code: NativeXcbConnectionCode) -> Option<Self> {
        NonzeroXcbConnectionCode::new(code).map(Self)
    }

    /// Returns the Error Code associated with this error.
    pub fn code(&self) -> NativeXcbConnectionCode {
        self.0.get()
    }

    /// Returns the Name of this error.
    pub fn name(&self) -> &'static str {
        log::xcb_connection_error_name(self.code())
    }
}

impl Error for XcbConnectionError {}

impl Debug for XcbConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        let name = self.name();
        write!(f, "Xcb Connection Error ({name}) [{code}]")
    }
}

impl Display for XcbConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        let name = self.name();
        write!(f, "Xcb Connection Error ({name}) [{code}]")
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Error-code for XCB Generic Errors.
pub(crate) type NativeXcbGenericErrorCode = u8;

/// Native error-type for XCB Generic Errors.
pub(crate) type NativeXcbGenericError = *mut sys::xcb_generic_error_t;

/// Nonzero-type for native XCB Generic Errors.
pub(crate) type NonzeroXcbGenericError = NonNull<sys::xcb_generic_error_t>;

/// A wrapper for XCB Generic Errors.
#[repr(transparent)]
pub struct XcbGenericError(NonzeroXcbGenericError);

/// A result for XCB Connections.
pub type XcbGenericResult<T> = Result<T, XcbGenericError>;

impl XcbGenericError {
    /// Attempts to construct a new `XcbGenericError`.
    pub(crate) fn new(code: NativeXcbGenericError) -> Option<Self> {
        NonzeroXcbGenericError::new(code).map(Self)
    }

    /// Returns the Error Code associated with this error.
    pub fn code(&self) -> NativeXcbGenericErrorCode {
        unsafe { self.0.as_ref() }.error_code
    }

    /// Returns the Name of this error.
    pub fn name(&self) -> &'static str {
        log::xcb_generic_error_name(unsafe { self.0.as_ref() })
    }
}

impl Error for XcbGenericError {}

impl Debug for XcbGenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        let name = self.name();
        write!(f, "Xcb Generic Error ({name}) [{code}]")
    }
}

impl Display for XcbGenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        let name = self.name();
        write!(f, "Xcb Generic Error ({name}) [{code}]")
    }
}

// ================================================================================================================================ //
