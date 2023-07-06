/*
 *  Crate: Wyn
 * Module: Linux - XCB
 */

//! Linux XCB bindings.
//!
//! # Dependencies
//! * <https://docs.rs/xcb-sys/latest/xcb_sys/index.html>
//!
//! # Documentation
//! * <https://xcb.freedesktop.org/XcbApi/>
//!     * <https://xcb.freedesktop.org/PublicApi/>
//!     * <https://xcb.freedesktop.org/ProtocolStubApi/>
//!     * <https://xcb.freedesktop.org/ProtocolExtensionApi/>
//! * <https://www.x.org/releases/current/doc/man/man3/>

// -------------------------------------------------------------------------------------------------------------------------------- //

use crate::common::c_types::*;

// ================================================================================================================================ //
// Macros
// -------------------------------------------------------------------------------------------------------------------------------- //

// ================================================================================================================================ //
// Types
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://xcb.freedesktop.org/PublicApi/#xcb_connection_t>
pub use ::xcb_sys::xcb_connection_t;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_get_setup>
pub use ::xcb_sys::xcb_setup_t;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_name_iterator_t>
pub use ::xcb_sys::xcb_screen_iterator_t;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_event_name_event_t>
pub use ::xcb_sys::xcb_generic_event_t;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_error_name_error_t>
pub use ::xcb_sys::xcb_generic_error_t;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_generate_id>
pub use ::xcb_sys::xcb_screen_t;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_generate_id>
pub use ::xcb_sys::xcb_window_t;

/// <https://xcb.freedesktop.org/tutorial/events/>
pub use ::xcb_sys::xcb_event_mask_t;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_name_cookie_t>
pub use ::xcb_sys::xcb_void_cookie_t;

/// ...
pub use ::xcb_sys::xcb_atom_t;

/// ...
pub use ::xcb_sys::xcb_client_message_data_t;

// Generic Event Types
pub use ::xcb_sys::xcb_button_press_event_t;
pub use ::xcb_sys::xcb_button_release_event_t;
pub use ::xcb_sys::xcb_circulate_notify_event_t;
pub use ::xcb_sys::xcb_circulate_request_event_t;
pub use ::xcb_sys::xcb_client_message_event_t;
pub use ::xcb_sys::xcb_colormap_notify_event_t;
pub use ::xcb_sys::xcb_configure_notify_event_t;
pub use ::xcb_sys::xcb_configure_request_event_t;
pub use ::xcb_sys::xcb_create_notify_event_t;
pub use ::xcb_sys::xcb_destroy_notify_event_t;
pub use ::xcb_sys::xcb_enter_notify_event_t;
pub use ::xcb_sys::xcb_expose_event_t;
pub use ::xcb_sys::xcb_focus_in_event_t;
pub use ::xcb_sys::xcb_focus_out_event_t;
pub use ::xcb_sys::xcb_ge_generic_event_t;
pub use ::xcb_sys::xcb_graphics_exposure_event_t;
pub use ::xcb_sys::xcb_gravity_notify_event_t;
pub use ::xcb_sys::xcb_key_press_event_t;
pub use ::xcb_sys::xcb_key_release_event_t;
pub use ::xcb_sys::xcb_keymap_notify_event_t;
pub use ::xcb_sys::xcb_leave_notify_event_t;
pub use ::xcb_sys::xcb_map_notify_event_t;
pub use ::xcb_sys::xcb_map_request_event_t;
pub use ::xcb_sys::xcb_mapping_notify_event_t;
pub use ::xcb_sys::xcb_motion_notify_event_t;
pub use ::xcb_sys::xcb_no_exposure_event_t;
pub use ::xcb_sys::xcb_property_notify_event_t;
pub use ::xcb_sys::xcb_reparent_notify_event_t;
pub use ::xcb_sys::xcb_resize_request_event_t;
pub use ::xcb_sys::xcb_selection_clear_event_t;
pub use ::xcb_sys::xcb_selection_notify_event_t;
pub use ::xcb_sys::xcb_selection_request_event_t;
pub use ::xcb_sys::xcb_unmap_notify_event_t;
pub use ::xcb_sys::xcb_visibility_notify_event_t;

// Generic Error Types
pub use ::xcb_sys::xcb_access_error_t;
pub use ::xcb_sys::xcb_alloc_error_t;
pub use ::xcb_sys::xcb_atom_error_t;
pub use ::xcb_sys::xcb_colormap_error_t;
pub use ::xcb_sys::xcb_cursor_error_t;
pub use ::xcb_sys::xcb_drawable_error_t;
pub use ::xcb_sys::xcb_font_error_t;
pub use ::xcb_sys::xcb_g_context_error_t;
pub use ::xcb_sys::xcb_id_choice_error_t;
pub use ::xcb_sys::xcb_implementation_error_t;
pub use ::xcb_sys::xcb_length_error_t;
pub use ::xcb_sys::xcb_match_error_t;
pub use ::xcb_sys::xcb_name_error_t;
pub use ::xcb_sys::xcb_pixmap_error_t;
pub use ::xcb_sys::xcb_window_error_t;

/// ...
pub use ::xcb_sys::xcb_atom_enum_t;

/// ...
pub use ::xcb_sys::xcb_config_window_t;

/// ...
pub use ::xcb_sys::xcb_stack_mode_t;

/// ...
pub use ::xcb_sys::xcb_get_window_attributes_reply_t;

/// ...
pub use ::xcb_sys::xcb_get_window_attributes_request_t;

/// ...
pub use ::xcb_sys::xcb_get_property_reply_t;

/// ...
pub use ::xcb_sys::xcb_get_geometry_reply_t;

/// ...
pub use ::xcb_sys::xcb_translate_coordinates_cookie_t;

/// ...
pub use ::xcb_sys::xcb_translate_coordinates_reply_t;

/// ...
pub use ::xcb_sys::xcb_translate_coordinates_request_t;

/// ...
pub use ::xcb_sys::xcb_query_tree_reply_t;

/// ...
pub use ::xcb_sys::xcb_point_t;

/// ...
pub use ::xcb_sys::xcb_rectangle_t;

/// ...
pub use ::xcb_sys::xcb_mod_mask_t;

/// ...
pub use ::xcb_sys::xcb_key_but_mask_t;

/// ...
pub use ::xcb_sys::xcb_ewmh_connection_t;

/// ...
pub use ::xcb_sys::xcb_ewmh_get_extents_reply_t;

/// ...
pub use ::xcb_sys::xcb_icccm_get_text_property_reply_t;

// ================================================================================================================================ //
// Functions
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://xcb.freedesktop.org/PublicApi/#xcb_connect>
pub use ::xcb_sys::xcb_connect;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_disconnect>
pub use ::xcb_sys::xcb_disconnect;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_connection_has_error>
pub use ::xcb_sys::xcb_connection_has_error;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_get_file_descriptor>
pub use ::xcb_sys::xcb_get_file_descriptor;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_request_check>
pub use ::xcb_sys::xcb_request_check;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_flush>
pub use ::xcb_sys::xcb_flush;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_generate_id>
pub use ::xcb_sys::xcb_generate_id;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_wait_for_event>
pub use ::xcb_sys::xcb_wait_for_event;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_poll_for_event>
pub use ::xcb_sys::xcb_poll_for_event;

/// <https://xcb.freedesktop.org/PublicApi/#xcb_get_setup>
pub use ::xcb_sys::xcb_get_setup;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_name_field_iterator>
pub use ::xcb_sys::xcb_setup_roots_iterator;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_name_next>
pub use ::xcb_sys::xcb_screen_next;

/// <https://xcb.freedesktop.org/ProtocolStubApi/#xcb_name_end>
pub use ::xcb_sys::xcb_screen_end;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_create_window.3.xhtml>
pub use ::xcb_sys::xcb_create_window;
pub use ::xcb_sys::xcb_create_window_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_destroy_window.3.xhtml>
pub use ::xcb_sys::xcb_destroy_window;
pub use ::xcb_sys::xcb_destroy_window_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_configure_window.3.xhtml>
pub use ::xcb_sys::xcb_configure_window;
pub use ::xcb_sys::xcb_configure_window_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_change_window_attributes.3.xhtml>
pub use ::xcb_sys::xcb_change_window_attributes;
pub use ::xcb_sys::xcb_change_window_attributes_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_get_window_attributes.3.xhtml>
pub use ::xcb_sys::xcb_get_window_attributes;
pub use ::xcb_sys::xcb_get_window_attributes_reply;
pub use ::xcb_sys::xcb_get_window_attributes_unchecked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_change_property.3.xhtml>
pub use ::xcb_sys::xcb_change_property;
pub use ::xcb_sys::xcb_change_property_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_get_property.3.xhtml>
pub use ::xcb_sys::xcb_get_property;
pub use ::xcb_sys::xcb_get_property_reply;
pub use ::xcb_sys::xcb_get_property_unchecked;
pub use ::xcb_sys::xcb_get_property_value;
pub use ::xcb_sys::xcb_get_property_value_end;
pub use ::xcb_sys::xcb_get_property_value_length;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_map_window.3.xhtml>
pub use ::xcb_sys::xcb_map_window;
pub use ::xcb_sys::xcb_map_window_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_unmap_window.3.xhtml>
pub use ::xcb_sys::xcb_unmap_window;
pub use ::xcb_sys::xcb_unmap_window_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_get_input_focus.3.xhtml>
pub use ::xcb_sys::xcb_get_input_focus;
pub use ::xcb_sys::xcb_get_input_focus_unchecked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_set_input_focus.3.xhtml>
pub use ::xcb_sys::xcb_set_input_focus;
pub use ::xcb_sys::xcb_set_input_focus_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_circulate_window.3.xhtml>
pub use ::xcb_sys::xcb_circulate_window;
pub use ::xcb_sys::xcb_circulate_window_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_send_event.3.xhtml>
pub use ::xcb_sys::xcb_send_event;
pub use ::xcb_sys::xcb_send_event_checked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_intern_atom.3.xhtml>
pub use ::xcb_sys::xcb_intern_atom;
pub use ::xcb_sys::xcb_intern_atom_reply;
pub use ::xcb_sys::xcb_intern_atom_unchecked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_get_geometry.3.xhtml>
pub use ::xcb_sys::xcb_get_geometry;
pub use ::xcb_sys::xcb_get_geometry_reply;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_translate_coordinates.3.xhtml>
pub use ::xcb_sys::xcb_translate_coordinates;
pub use ::xcb_sys::xcb_translate_coordinates_reply;
pub use ::xcb_sys::xcb_translate_coordinates_unchecked;

/// <https://www.x.org/releases/current/doc/man/man3/xcb_query_tree.3.xhtml>
pub use ::xcb_sys::xcb_query_tree;
pub use ::xcb_sys::xcb_query_tree_reply;
pub use ::xcb_sys::xcb_query_tree_unchecked;

/// ...
pub use ::xcb_sys::xcb_ewmh_init_atoms;
pub use ::xcb_sys::xcb_ewmh_init_atoms_replies;

/// ...
pub use ::xcb_sys::xcb_ewmh_set_frame_extents;
pub use ::xcb_sys::xcb_ewmh_set_frame_extents_checked;

/// ...
pub use ::xcb_sys::xcb_ewmh_get_frame_extents;
pub use ::xcb_sys::xcb_ewmh_get_frame_extents_from_reply;
pub use ::xcb_sys::xcb_ewmh_get_frame_extents_reply;
pub use ::xcb_sys::xcb_ewmh_get_frame_extents_unchecked;

/// ...
pub use ::xcb_sys::xcb_icccm_set_wm_name;
pub use ::xcb_sys::xcb_icccm_set_wm_name_checked;

/// ...
pub use ::xcb_sys::xcb_icccm_get_wm_name;
pub use ::xcb_sys::xcb_icccm_get_wm_name_reply;
pub use ::xcb_sys::xcb_icccm_get_wm_name_unchecked;

// ================================================================================================================================ //
// Constants
// -------------------------------------------------------------------------------------------------------------------------------- //

pub use ::xcb_sys::XCB_NONE;

pub use ::xcb_sys::XCB_CONN_CLOSED_EXT_NOTSUPPORTED;
pub use ::xcb_sys::XCB_CONN_CLOSED_FDPASSING_FAILED;
pub use ::xcb_sys::XCB_CONN_CLOSED_INVALID_SCREEN;
pub use ::xcb_sys::XCB_CONN_CLOSED_MEM_INSUFFICIENT;
pub use ::xcb_sys::XCB_CONN_CLOSED_PARSE_ERR;
pub use ::xcb_sys::XCB_CONN_CLOSED_REQ_LEN_EXCEED;
pub use ::xcb_sys::XCB_CONN_ERROR;

pub use ::xcb_sys::XCB_COPY_FROM_PARENT;

pub use ::xcb_sys::XCB_WINDOW_CLASS_COPY_FROM_PARENT;
pub use ::xcb_sys::XCB_WINDOW_CLASS_INPUT_ONLY;
pub use ::xcb_sys::XCB_WINDOW_CLASS_INPUT_OUTPUT;

pub use ::xcb_sys::XCB_CW_BACKING_PIXEL;
pub use ::xcb_sys::XCB_CW_BACKING_PLANES;
pub use ::xcb_sys::XCB_CW_BACKING_STORE;
pub use ::xcb_sys::XCB_CW_BACK_PIXEL;
pub use ::xcb_sys::XCB_CW_BACK_PIXMAP;
pub use ::xcb_sys::XCB_CW_BIT_GRAVITY;
pub use ::xcb_sys::XCB_CW_BORDER_PIXEL;
pub use ::xcb_sys::XCB_CW_BORDER_PIXMAP;
pub use ::xcb_sys::XCB_CW_COLORMAP;
pub use ::xcb_sys::XCB_CW_CURSOR;
pub use ::xcb_sys::XCB_CW_DONT_PROPAGATE;
pub use ::xcb_sys::XCB_CW_EVENT_MASK;
pub use ::xcb_sys::XCB_CW_OVERRIDE_REDIRECT;
pub use ::xcb_sys::XCB_CW_SAVE_UNDER;
pub use ::xcb_sys::XCB_CW_WIN_GRAVITY;

pub use ::xcb_sys::XCB_PROP_MODE_APPEND;
pub use ::xcb_sys::XCB_PROP_MODE_PREPEND;
pub use ::xcb_sys::XCB_PROP_MODE_REPLACE;

pub use ::xcb_sys::XCB_CONFIG_WINDOW_BORDER_WIDTH;
pub use ::xcb_sys::XCB_CONFIG_WINDOW_HEIGHT;
pub use ::xcb_sys::XCB_CONFIG_WINDOW_SIBLING;
pub use ::xcb_sys::XCB_CONFIG_WINDOW_STACK_MODE;
pub use ::xcb_sys::XCB_CONFIG_WINDOW_WIDTH;
pub use ::xcb_sys::XCB_CONFIG_WINDOW_X;
pub use ::xcb_sys::XCB_CONFIG_WINDOW_Y;

pub use ::xcb_sys::XCB_STACK_MODE_ABOVE;
pub use ::xcb_sys::XCB_STACK_MODE_BELOW;
pub use ::xcb_sys::XCB_STACK_MODE_BOTTOM_IF;
pub use ::xcb_sys::XCB_STACK_MODE_OPPOSITE;
pub use ::xcb_sys::XCB_STACK_MODE_TOP_IF;

pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_1_MOTION;
pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_2_MOTION;
pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_3_MOTION;
pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_4_MOTION;
pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_5_MOTION;
pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_MOTION;
pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_PRESS;
pub use ::xcb_sys::XCB_EVENT_MASK_BUTTON_RELEASE;
pub use ::xcb_sys::XCB_EVENT_MASK_COLOR_MAP_CHANGE;
pub use ::xcb_sys::XCB_EVENT_MASK_ENTER_WINDOW;
pub use ::xcb_sys::XCB_EVENT_MASK_EXPOSURE;
pub use ::xcb_sys::XCB_EVENT_MASK_FOCUS_CHANGE;
pub use ::xcb_sys::XCB_EVENT_MASK_KEYMAP_STATE;
pub use ::xcb_sys::XCB_EVENT_MASK_KEY_PRESS;
pub use ::xcb_sys::XCB_EVENT_MASK_KEY_RELEASE;
pub use ::xcb_sys::XCB_EVENT_MASK_LEAVE_WINDOW;
pub use ::xcb_sys::XCB_EVENT_MASK_NO_EVENT;
pub use ::xcb_sys::XCB_EVENT_MASK_OWNER_GRAB_BUTTON;
pub use ::xcb_sys::XCB_EVENT_MASK_POINTER_MOTION;
pub use ::xcb_sys::XCB_EVENT_MASK_POINTER_MOTION_HINT;
pub use ::xcb_sys::XCB_EVENT_MASK_PROPERTY_CHANGE;
pub use ::xcb_sys::XCB_EVENT_MASK_RESIZE_REDIRECT;
pub use ::xcb_sys::XCB_EVENT_MASK_STRUCTURE_NOTIFY;
pub use ::xcb_sys::XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY;
pub use ::xcb_sys::XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT;
pub use ::xcb_sys::XCB_EVENT_MASK_VISIBILITY_CHANGE;

pub use ::xcb_sys::XCB_INPUT_FOCUS_FOLLOW_KEYBOARD;
pub use ::xcb_sys::XCB_INPUT_FOCUS_NONE;
pub use ::xcb_sys::XCB_INPUT_FOCUS_PARENT;
pub use ::xcb_sys::XCB_INPUT_FOCUS_POINTER_ROOT;

pub use ::xcb_sys::XCB_CURRENT_TIME;

pub use ::xcb_sys::XCB_CIRCULATE_LOWER_HIGHEST;
pub use ::xcb_sys::XCB_CIRCULATE_RAISE_LOWEST;

// Generic Event Constants
pub use ::xcb_sys::XCB_BUTTON_PRESS;
pub use ::xcb_sys::XCB_BUTTON_RELEASE;
pub use ::xcb_sys::XCB_CIRCULATE_NOTIFY;
pub use ::xcb_sys::XCB_CIRCULATE_REQUEST;
pub use ::xcb_sys::XCB_CLIENT_MESSAGE;
pub use ::xcb_sys::XCB_COLORMAP_NOTIFY;
pub use ::xcb_sys::XCB_CONFIGURE_NOTIFY;
pub use ::xcb_sys::XCB_CONFIGURE_REQUEST;
pub use ::xcb_sys::XCB_CREATE_NOTIFY;
pub use ::xcb_sys::XCB_DESTROY_NOTIFY;
pub use ::xcb_sys::XCB_ENTER_NOTIFY;
pub use ::xcb_sys::XCB_EXPOSE;
pub use ::xcb_sys::XCB_FOCUS_IN;
pub use ::xcb_sys::XCB_FOCUS_OUT;
pub use ::xcb_sys::XCB_GE_GENERIC;
pub use ::xcb_sys::XCB_GRAPHICS_EXPOSURE;
pub use ::xcb_sys::XCB_GRAVITY_NOTIFY;
pub use ::xcb_sys::XCB_KEYMAP_NOTIFY;
pub use ::xcb_sys::XCB_KEY_PRESS;
pub use ::xcb_sys::XCB_KEY_RELEASE;
pub use ::xcb_sys::XCB_LEAVE_NOTIFY;
pub use ::xcb_sys::XCB_MAPPING_NOTIFY;
pub use ::xcb_sys::XCB_MAP_NOTIFY;
pub use ::xcb_sys::XCB_MAP_REQUEST;
pub use ::xcb_sys::XCB_MOTION_NOTIFY;
pub use ::xcb_sys::XCB_NO_EXPOSURE;
pub use ::xcb_sys::XCB_PROPERTY_NOTIFY;
pub use ::xcb_sys::XCB_REPARENT_NOTIFY;
pub use ::xcb_sys::XCB_RESIZE_REQUEST;
pub use ::xcb_sys::XCB_SELECTION_CLEAR;
pub use ::xcb_sys::XCB_SELECTION_NOTIFY;
pub use ::xcb_sys::XCB_SELECTION_REQUEST;
pub use ::xcb_sys::XCB_UNMAP_NOTIFY;
pub use ::xcb_sys::XCB_VISIBILITY_NOTIFY;

// Generic Error Constants
pub use ::xcb_sys::XCB_ACCESS;
pub use ::xcb_sys::XCB_ALLOC;
pub use ::xcb_sys::XCB_ATOM;
pub use ::xcb_sys::XCB_COLORMAP;
pub use ::xcb_sys::XCB_CURSOR;
pub use ::xcb_sys::XCB_DRAWABLE;
pub use ::xcb_sys::XCB_FONT;
pub use ::xcb_sys::XCB_G_CONTEXT;
pub use ::xcb_sys::XCB_ID_CHOICE;
pub use ::xcb_sys::XCB_IMPLEMENTATION;
pub use ::xcb_sys::XCB_LENGTH;
pub use ::xcb_sys::XCB_MATCH;
pub use ::xcb_sys::XCB_NAME;
pub use ::xcb_sys::XCB_PIXMAP;
pub use ::xcb_sys::XCB_REQUEST;
pub use ::xcb_sys::XCB_VALUE;
pub use ::xcb_sys::XCB_WINDOW;

pub use ::xcb_sys::XCB_ATOM_ANY;
pub use ::xcb_sys::XCB_ATOM_ARC;
pub use ::xcb_sys::XCB_ATOM_ATOM;
pub use ::xcb_sys::XCB_ATOM_BITMAP;
pub use ::xcb_sys::XCB_ATOM_CAP_HEIGHT;
pub use ::xcb_sys::XCB_ATOM_CARDINAL;
pub use ::xcb_sys::XCB_ATOM_COLORMAP;
pub use ::xcb_sys::XCB_ATOM_COPYRIGHT;
pub use ::xcb_sys::XCB_ATOM_CURSOR;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER0;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER1;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER2;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER3;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER4;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER5;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER6;
pub use ::xcb_sys::XCB_ATOM_CUT_BUFFER7;
pub use ::xcb_sys::XCB_ATOM_DRAWABLE;
pub use ::xcb_sys::XCB_ATOM_END_SPACE;
pub use ::xcb_sys::XCB_ATOM_FAMILY_NAME;
pub use ::xcb_sys::XCB_ATOM_FONT;
pub use ::xcb_sys::XCB_ATOM_FONT_NAME;
pub use ::xcb_sys::XCB_ATOM_FULL_NAME;
pub use ::xcb_sys::XCB_ATOM_INTEGER;
pub use ::xcb_sys::XCB_ATOM_ITALIC_ANGLE;
pub use ::xcb_sys::XCB_ATOM_MAX_SPACE;
pub use ::xcb_sys::XCB_ATOM_MIN_SPACE;
pub use ::xcb_sys::XCB_ATOM_NONE;
pub use ::xcb_sys::XCB_ATOM_NORM_SPACE;
pub use ::xcb_sys::XCB_ATOM_NOTICE;
pub use ::xcb_sys::XCB_ATOM_PIXMAP;
pub use ::xcb_sys::XCB_ATOM_POINT;
pub use ::xcb_sys::XCB_ATOM_POINT_SIZE;
pub use ::xcb_sys::XCB_ATOM_PRIMARY;
pub use ::xcb_sys::XCB_ATOM_QUAD_WIDTH;
pub use ::xcb_sys::XCB_ATOM_RECTANGLE;
pub use ::xcb_sys::XCB_ATOM_RESOLUTION;
pub use ::xcb_sys::XCB_ATOM_RESOURCE_MANAGER;
pub use ::xcb_sys::XCB_ATOM_RGB_BEST_MAP;
pub use ::xcb_sys::XCB_ATOM_RGB_BLUE_MAP;
pub use ::xcb_sys::XCB_ATOM_RGB_COLOR_MAP;
pub use ::xcb_sys::XCB_ATOM_RGB_DEFAULT_MAP;
pub use ::xcb_sys::XCB_ATOM_RGB_GRAY_MAP;
pub use ::xcb_sys::XCB_ATOM_RGB_GREEN_MAP;
pub use ::xcb_sys::XCB_ATOM_RGB_RED_MAP;
pub use ::xcb_sys::XCB_ATOM_SECONDARY;
pub use ::xcb_sys::XCB_ATOM_STRIKEOUT_ASCENT;
pub use ::xcb_sys::XCB_ATOM_STRIKEOUT_DESCENT;
pub use ::xcb_sys::XCB_ATOM_STRING;
pub use ::xcb_sys::XCB_ATOM_SUBSCRIPT_X;
pub use ::xcb_sys::XCB_ATOM_SUBSCRIPT_Y;
pub use ::xcb_sys::XCB_ATOM_SUPERSCRIPT_X;
pub use ::xcb_sys::XCB_ATOM_SUPERSCRIPT_Y;
pub use ::xcb_sys::XCB_ATOM_UNDERLINE_POSITION;
pub use ::xcb_sys::XCB_ATOM_UNDERLINE_THICKNESS;
pub use ::xcb_sys::XCB_ATOM_VISUALID;
pub use ::xcb_sys::XCB_ATOM_WEIGHT;
pub use ::xcb_sys::XCB_ATOM_WINDOW;
pub use ::xcb_sys::XCB_ATOM_WM_CLASS;
pub use ::xcb_sys::XCB_ATOM_WM_CLIENT_MACHINE;
pub use ::xcb_sys::XCB_ATOM_WM_COMMAND;
pub use ::xcb_sys::XCB_ATOM_WM_HINTS;
pub use ::xcb_sys::XCB_ATOM_WM_ICON_NAME;
pub use ::xcb_sys::XCB_ATOM_WM_ICON_SIZE;
pub use ::xcb_sys::XCB_ATOM_WM_NAME;
pub use ::xcb_sys::XCB_ATOM_WM_NORMAL_HINTS;
pub use ::xcb_sys::XCB_ATOM_WM_SIZE_HINTS;
pub use ::xcb_sys::XCB_ATOM_WM_TRANSIENT_FOR;
pub use ::xcb_sys::XCB_ATOM_WM_ZOOM_HINTS;
pub use ::xcb_sys::XCB_ATOM_X_HEIGHT;

pub use ::xcb_sys::XCB_MOD_MASK_1;
pub use ::xcb_sys::XCB_MOD_MASK_2;
pub use ::xcb_sys::XCB_MOD_MASK_3;
pub use ::xcb_sys::XCB_MOD_MASK_4;
pub use ::xcb_sys::XCB_MOD_MASK_5;
pub use ::xcb_sys::XCB_MOD_MASK_ANY;
pub use ::xcb_sys::XCB_MOD_MASK_CONTROL;
pub use ::xcb_sys::XCB_MOD_MASK_LOCK;
pub use ::xcb_sys::XCB_MOD_MASK_SHIFT;

pub use ::xcb_sys::XCB_KEY_BUT_MASK_BUTTON_1;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_BUTTON_2;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_BUTTON_3;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_BUTTON_4;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_BUTTON_5;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_CONTROL;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_LOCK;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_MOD_1;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_MOD_2;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_MOD_3;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_MOD_4;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_MOD_5;
pub use ::xcb_sys::XCB_KEY_BUT_MASK_SHIFT;

// ================================================================================================================================ //
