/*
 *  Crate: Wyn
 * Module: X11 - Log
 */

//! Module for textualizing or printing out events, errors, and other platform-specific data.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::errors::*;

// ================================================================================================================================ //

/// Default string returned when a variant is UNKNOWN.
const DEFAULT_STRING: &str = "<N/A>";

// ================================================================================================================================ //

/// Returns the Name of the Event, if known.
pub const fn xcb_generic_event_name(event: &sys::xcb_generic_event_t) -> &'static str {
    let variant = event.response_type & 0x7F;

    match variant as u32 {
        sys::XCB_BUTTON_PRESS => "XCB_BUTTON_PRESS_EVENT",
        sys::XCB_BUTTON_RELEASE => "XCB_BUTTON_RELEASE_EVENT",
        sys::XCB_CIRCULATE_NOTIFY => "XCB_CIRCULATE_NOTIFY_EVENT",
        sys::XCB_CIRCULATE_REQUEST => "XCB_CIRCULATE_REQUEST_EVENT",
        sys::XCB_CLIENT_MESSAGE => "XCB_CLIENT_MESSAGE_EVENT",
        sys::XCB_COLORMAP_NOTIFY => "XCB_COLORMAP_NOTIFY_EVENT",
        sys::XCB_CONFIGURE_NOTIFY => "XCB_CONFIGURE_NOTIFY_EVENT",
        sys::XCB_CONFIGURE_REQUEST => "XCB_CONFIGURE_REQUEST_EVENT",
        sys::XCB_CREATE_NOTIFY => "XCB_CREATE_NOTIFY_EVENT",
        sys::XCB_DESTROY_NOTIFY => "XCB_DESTROY_NOTIFY_EVENT",
        sys::XCB_ENTER_NOTIFY => "XCB_ENTER_NOTIFY_EVENT",
        sys::XCB_EXPOSE => "XCB_EXPOSE_EVENT",
        sys::XCB_FOCUS_IN => "XCB_FOCUS_IN_EVENT",
        sys::XCB_FOCUS_OUT => "XCB_FOCUS_OUT_EVENT",
        sys::XCB_GE_GENERIC => "XCB_GE_GENERIC_EVENT",
        sys::XCB_GRAPHICS_EXPOSURE => "XCB_GRAPHICS_EXPOSURE_EVENT",
        sys::XCB_GRAVITY_NOTIFY => "XCB_GRAVITY_NOTIFY_EVENT",
        sys::XCB_KEYMAP_NOTIFY => "XCB_KEYMAP_NOTIFY_EVENT",
        sys::XCB_KEY_PRESS => "XCB_KEY_PRESS_EVENT",
        sys::XCB_KEY_RELEASE => "XCB_KEY_RELEASE_EVENT",
        sys::XCB_LEAVE_NOTIFY => "XCB_LEAVE_NOTIFY_EVENT",
        sys::XCB_MAPPING_NOTIFY => "XCB_MAPPING_NOTIFY_EVENT",
        sys::XCB_MAP_NOTIFY => "XCB_MAP_NOTIFY_EVENT",
        sys::XCB_MAP_REQUEST => "XCB_MAP_REQUEST_EVENT",
        sys::XCB_MOTION_NOTIFY => "XCB_MOTION_NOTIFY_EVENT",
        sys::XCB_NO_EXPOSURE => "XCB_NO_EXPOSURE_EVENT",
        sys::XCB_PROPERTY_NOTIFY => "XCB_PROPERTY_NOTIFY_EVENT",
        sys::XCB_REPARENT_NOTIFY => "XCB_REPARENT_NOTIFY_EVENT",
        sys::XCB_RESIZE_REQUEST => "XCB_RESIZE_REQUEST_EVENT",
        sys::XCB_SELECTION_CLEAR => "XCB_SELECTION_CLEAR_EVENT",
        sys::XCB_SELECTION_NOTIFY => "XCB_SELECTION_NOTIFY_EVENT",
        sys::XCB_SELECTION_REQUEST => "XCB_SELECTION_REQUEST_EVENT",
        sys::XCB_UNMAP_NOTIFY => "XCB_UNMAP_NOTIFY_EVENT",
        sys::XCB_VISIBILITY_NOTIFY => "XCB_VISIBILITY_NOTIFY_EVENT",
        _ => DEFAULT_STRING,
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Returns the Name of the Error, if known.
pub const fn xcb_generic_error_name(error: &sys::xcb_generic_error_t) -> &'static str {
    let variant = error.error_code;

    match variant as u32 {
        sys::XCB_ACCESS => "XCB_ACCESS_ERROR",
        sys::XCB_ALLOC => "XCB_ALLOC_ERROR",
        sys::XCB_ATOM => "XCB_ATOM_ERROR",
        sys::XCB_COLORMAP => "XCB_COLORMAP_ERROR",
        sys::XCB_CURSOR => "XCB_CURSOR_ERROR",
        sys::XCB_DRAWABLE => "XCB_DRAWABLE_ERROR",
        sys::XCB_FONT => "XCB_FONT_ERROR",
        sys::XCB_G_CONTEXT => "XCB_G_CONTEXT_ERROR",
        sys::XCB_ID_CHOICE => "XCB_ID_CHOICE_ERROR",
        sys::XCB_IMPLEMENTATION => "XCB_IMPLEMENTATION_ERROR",
        sys::XCB_LENGTH => "XCB_LENGTH_ERROR",
        sys::XCB_MATCH => "XCB_MATCH_ERROR",
        sys::XCB_NAME => "XCB_NAME_ERROR",
        sys::XCB_PIXMAP => "XCB_PIXMAP_ERROR",
        sys::XCB_REQUEST => "XCB_REQUEST_ERROR",
        sys::XCB_VALUE => "XCB_VALUE_ERROR",
        sys::XCB_WINDOW => "XCB_WINDOW_ERROR",
        _ => DEFAULT_STRING,
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Returns the Name of the Error, if known.
pub const fn xcb_connection_error_name(error: NativeXcbConnectionCode) -> &'static str {
    match error as u32 {
        sys::XCB_CONN_CLOSED_EXT_NOTSUPPORTED => "XCB_CONN_CLOSED_EXT_NOTSUPPORTED",
        sys::XCB_CONN_CLOSED_FDPASSING_FAILED => "XCB_CONN_CLOSED_FDPASSING_FAILED",
        sys::XCB_CONN_CLOSED_INVALID_SCREEN => "XCB_CONN_CLOSED_INVALID_SCREEN",
        sys::XCB_CONN_CLOSED_MEM_INSUFFICIENT => "XCB_CONN_CLOSED_MEM_INSUFFICIENT",
        sys::XCB_CONN_CLOSED_PARSE_ERR => "XCB_CONN_CLOSED_PARSE_ERR",
        sys::XCB_CONN_CLOSED_REQ_LEN_EXCEED => "XCB_CONN_CLOSED_REQ_LEN_EXCEED",
        sys::XCB_CONN_ERROR => "XCB_CONN_ERROR",
        _ => DEFAULT_STRING,
    }
}

// ================================================================================================================================ //
