/*
 *  Crate: Wyn
 * Module: X11 - Window
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::errors::XcbGenericError;
use super::event_loop::EventLoop;
use super::types::{Coord, Extent, Rect};

// ================================================================================================================================ //

/// Mask for All XCB Events.
const XCB_EVENT_MASK_ALL: sys::xcb_event_mask_t = sys::XCB_EVENT_MASK_NO_EVENT
    | sys::XCB_EVENT_MASK_KEY_PRESS
    | sys::XCB_EVENT_MASK_KEY_RELEASE
    | sys::XCB_EVENT_MASK_BUTTON_PRESS
    | sys::XCB_EVENT_MASK_BUTTON_RELEASE
    | sys::XCB_EVENT_MASK_ENTER_WINDOW
    | sys::XCB_EVENT_MASK_LEAVE_WINDOW
    | sys::XCB_EVENT_MASK_POINTER_MOTION
    //| sys::XCB_EVENT_MASK_POINTER_MOTION_HINT
    | sys::XCB_EVENT_MASK_BUTTON_1_MOTION
    | sys::XCB_EVENT_MASK_BUTTON_2_MOTION
    | sys::XCB_EVENT_MASK_BUTTON_3_MOTION
    | sys::XCB_EVENT_MASK_BUTTON_4_MOTION
    | sys::XCB_EVENT_MASK_BUTTON_5_MOTION
    | sys::XCB_EVENT_MASK_BUTTON_MOTION
    | sys::XCB_EVENT_MASK_KEYMAP_STATE
    | sys::XCB_EVENT_MASK_EXPOSURE
    | sys::XCB_EVENT_MASK_VISIBILITY_CHANGE
    | sys::XCB_EVENT_MASK_STRUCTURE_NOTIFY
    //| sys::XCB_EVENT_MASK_RESIZE_REDIRECT
    | sys::XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY
    //| sys::XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT
    | sys::XCB_EVENT_MASK_FOCUS_CHANGE
    | sys::XCB_EVENT_MASK_PROPERTY_CHANGE
    | sys::XCB_EVENT_MASK_COLOR_MAP_CHANGE
    | sys::XCB_EVENT_MASK_OWNER_GRAB_BUTTON;

// ================================================================================================================================ //

/// Underlying OS Handle to a Window.
pub type WindowHandle = sys::xcb_window_t;

/// Nonzero wrapper for Window Handles.
type NonZeroWindowHandle = NonZero<sys::xcb_window_t>;

/// Wrapper for Window objects.
#[repr(transparent)]
pub struct Window(NonZeroWindowHandle);

// ================================================================================================================================ //

impl Window {
    /// Returns the Native OS Handle to this Window.
    pub fn handle(&self) -> WindowHandle {
        self.0.get()
    }
}

impl From<NonZeroWindowHandle> for Window {
    fn from(handle: NonZeroWindowHandle) -> Self {
        Self(handle)
    }
}

impl TryFrom<WindowHandle> for Window {
    type Error = ();

    fn try_from(handle: WindowHandle) -> Result<Self, Self::Error> {
        match NonZeroWindowHandle::new(handle) {
            Some(handle) => Ok(Self(handle)),
            None => Err(()),
        }
    }
}

// ================================================================================================================================ //

/// Open - Close
impl Window {
    // ---------------------------------------------------------------- //

    /// Attempts to open a new Window.
    pub fn open(events: &EventLoop) -> Window {
        let window = Self::internal_open(events);
        window.internal_set_attributes(events);
        window
    }

    /// Returns whether or not the Window is Open.
    pub fn is_open(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    /// Opens a window, without modifying attributes/properties.
    fn internal_open(events: &EventLoop) -> Window {
        let screen = events.connection.screen();

        let handle = unsafe { sys::xcb_generate_id(events.connection.handle) };

        let value_mask = sys::XCB_CW_EVENT_MASK;
        let value_list = [/* XCB_CW_EVENT_MASK */ XCB_EVENT_MASK_ALL];

        let cookie = unsafe {
            sys::xcb_create_window_checked(
                events.connection.handle,
                sys::XCB_COPY_FROM_PARENT as u8,
                handle,
                screen.root,
                0,
                0,
                640,
                480,
                16,
                sys::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual,
                value_mask,
                void_of!(value_list),
            )
        };
        events.connection.check_cookie(cookie).unwrap();

        Window::try_from(handle).expect("Invalid Window Handle.")
    }

    /// Sets the atributes/properties of a newly created Window.
    fn internal_set_attributes(&self, events: &EventLoop) {
        let screen = events.connection.screen();

        let cookie = unsafe {
            let list = [events.atoms.wm_delete_window.get()];
            let data = void_of!(list);
            let data_len = list.len() as u32;

            sys::xcb_change_property_checked(
                events.connection.handle,
                sys::XCB_PROP_MODE_REPLACE as u8,
                self.handle(),
                events.atoms.ewmh_ref().WM_PROTOCOLS,
                sys::XCB_ATOM_ATOM,
                32,
                data_len,
                data,
            )
        };
        events.connection.check_cookie(cookie).unwrap();

        let value_mask = sys::XCB_CW_BACK_PIXEL | sys::XCB_CW_BACKING_PIXEL;
        let value_list = [screen.black_pixel, screen.white_pixel];
        let cookie = unsafe {
            sys::xcb_change_window_attributes_checked(
                events.connection.handle,
                self.handle(),
                value_mask,
                void_of!(value_list),
            )
        };
        events.connection.check_cookie(cookie).unwrap();
    }

    // ---------------------------------------------------------------- //

    /// Closes the Window.
    pub fn close(self, events: &EventLoop) {
        self.internal_close(events);
        events.connection.flush();
    }

    /// Returns whether or not the Window is Closed.
    pub fn is_closed(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    /// Closes the Window, without flushing.
    fn internal_close(&self, events: &EventLoop) {
        let cookie =
            unsafe { sys::xcb_destroy_window_checked(events.connection.handle, self.handle()) };
        events.connection.check_cookie(cookie).unwrap();
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Focus - Show - Hide
impl Window {
    // ---------------------------------------------------------------- //

    /// Makes the Window visible and gives it focus.
    pub fn focus(&self, events: &EventLoop) {
        self.internal_show(events);
        self.internal_raise(events);

        if let Some(atom) =
            event_loop::NonzeroXcbAtom::new(events.atoms.ewmh_ref()._NET_ACTIVE_WINDOW)
        {
            self.internal_focus_wm(events, atom.get())
        } else {
            self.internal_focus_fallback(events);
        }

        events.connection.flush();
    }

    /// Returns whether or not the Window is Focused.
    pub fn is_focused(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    /// Focuses the Window, with help from the Window Manager, without flushing.
    fn internal_focus_wm(&self, events: &EventLoop, net_active_window: sys::xcb_atom_t) {
        let screen = events.connection.screen();

        let event = sys::xcb_client_message_event_t {
            response_type: sys::XCB_CLIENT_MESSAGE as u8,
            window: self.handle(),
            format: 32,
            type_: net_active_window,
            data: sys::xcb_client_message_data_t {
                data32: [1, 0, 0, 0, 0],
            },
            sequence: 0,
        };

        let prop = false as u8;
        let dest = screen.root;
        let event_mask =
            sys::XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY | sys::XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT;

        let event_data = bytes_of!(event);

        let cookie = unsafe {
            sys::xcb_send_event_checked(
                events.connection.handle,
                prop,
                dest,
                event_mask,
                event_data,
            )
        };

        events.connection.check_cookie(cookie).unwrap();
    }

    /// Focuses the Window, without flushing.
    fn internal_focus_fallback(&self, events: &EventLoop) {
        let cookie = unsafe {
            sys::xcb_set_input_focus_checked(
                events.connection.handle,
                sys::XCB_INPUT_FOCUS_PARENT as u8,
                self.handle(),
                sys::XCB_CURRENT_TIME,
            )
        };

        events.connection.check_cookie(cookie).unwrap();
    }

    // ---------------------------------------------------------------- //

    /// Makes the Window visible.
    pub fn show(&self, events: &EventLoop) {
        self.internal_show(events);
        self.internal_raise(events);
        events.connection.flush();
    }

    /// Returns whether or not the Window is Visible.
    pub fn is_visible(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    /// Shows the window, without flushing.
    fn internal_show(&self, events: &EventLoop) {
        let cookie =
            unsafe { sys::xcb_map_window_checked(events.connection.handle, self.handle()) };
        events.connection.check_cookie(cookie).unwrap();
    }

    /// Raises the window, without flushing.
    fn internal_raise(&self, events: &EventLoop) {
        let cookie = unsafe {
            sys::xcb_circulate_window_checked(
                events.connection.handle,
                sys::XCB_CIRCULATE_RAISE_LOWEST as u8,
                self.handle(),
            )
        };
        events.connection.check_cookie(cookie).unwrap();
    }

    // ---------------------------------------------------------------- //

    /// Makes the Window invisible.
    pub fn hide(&self, events: &EventLoop) {
        self.internal_hide(events);
        events.connection.flush();
    }

    /// Returns whether or not the Window is Hidden.
    pub fn is_hidden(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    /// Hides the window, without flushing.
    fn internal_hide(&self, events: &EventLoop) {
        let cookie =
            unsafe { sys::xcb_unmap_window_checked(events.connection.handle, self.handle()) };
        events.connection.check_cookie(cookie).unwrap();
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Rename
impl Window {
    /// Sets the Name of the Window.
    pub fn rename(&self, events: &EventLoop, name: &str) {
        self.internal_rename(events, name);
        events.connection.flush();
    }

    /// Renames the window, without flushing.
    pub fn internal_rename(&self, events: &EventLoop, name: &str) {
        let cookie = unsafe {
            sys::xcb_icccm_set_wm_name_checked(
                events.connection.handle,
                self.handle(),
                events.atoms.ewmh_ref().UTF8_STRING,
                8,
                name.len() as u32,
                name.as_ptr() as *const c_char,
            )
        };
        events.connection.check_cookie(cookie).unwrap();
    }

    /// Gets the Name of the Window.
    pub fn name(&self, events: &EventLoop) -> String {
        let mut error = null_mut();
        let mut prop = unsafe { zeroed() };

        let cookie = unsafe { sys::xcb_icccm_get_wm_name(events.connection.handle, self.handle()) };
        let reply = unsafe {
            sys::xcb_icccm_get_wm_name_reply(
                events.connection.handle,
                cookie,
                addr_of_mut!(prop),
                addr_of_mut!(error),
            )
        };
        assert!(XcbGenericError::new(error).is_none());
        assert_eq!(reply, 1);

        assert_eq!(prop.encoding, events.atoms.ewmh_ref().UTF8_STRING);

        let len = prop.name_len as usize;
        let dat = prop.name as *const u8;
        assert!(!dat.is_null());

        let slice = unsafe { core::slice::from_raw_parts(dat, len) };
        String::from_utf8_lossy(slice).to_string()
    }
}

// ================================================================================================================================ //

/// Reposition
impl Window {
    // ---------------------------------------------------------------- //

    /// Sets the rectangle of the Inner-Content of the Window.
    pub fn reposition_content(&self, events: &EventLoop, rect: Rect) {
        let mask = sys::XCB_CONFIG_WINDOW_X
            | sys::XCB_CONFIG_WINDOW_Y
            | sys::XCB_CONFIG_WINDOW_WIDTH
            | sys::XCB_CONFIG_WINDOW_HEIGHT;

        let mut list = [
            /* XCB_CONFIG_WINDOW_X */ rect.origin.x as i32,
            /* XCB_CONFIG_WINDOW_Y */ rect.origin.y as i32,
            /* XCB_CONFIG_WINDOW_WIDTH */ rect.size.w as i32,
            /* XCB_CONFIG_WINDOW_HEIGHT*/ rect.size.h as i32,
        ];

        if let Some(margins) = self.internal_margins(events) {
            list[0] -= margins[0] as i32;
            list[1] -= margins[2] as i32;
        }

        let cookie = unsafe {
            sys::xcb_configure_window_checked(
                events.connection.handle,
                self.handle(),
                mask as u16,
                void_of!(list),
            )
        };
        events.connection.check_cookie(cookie).unwrap();

        events.connection.flush();
    }

    /// Retrieves [Left, Right, Top, Bottom] Window Margins.
    fn internal_margins(&self, events: &EventLoop) -> Option<[u32; 4]> {
        // let mut error = null_mut();

        // let cookie = unsafe {
        //     sys::xcb_get_property(
        //         events.connection.handle,
        //         false as u8,
        //         self.handle(),
        //         net_frame_extents,
        //         sys::XCB_ATOM_CARDINAL,
        //         0,
        //         4,
        //     )
        // };

        // let reply = unsafe {
        //     sys::xcb_get_property_reply(events.connection.handle, cookie, addr_of_mut!(error))
        // };
        // let _reply_free = defer(|| {
        //     if !reply.is_null() {
        //         unsafe { sys::free(reply as *mut c_void) };
        //     }
        // });

        // assert!(XcbGenericError::new(error).is_none());

        // match unsafe { reply.as_ref() } {
        //     Some(_rep) => {
        //         let dat = unsafe { sys::xcb_get_property_value(reply) };
        //         assert!(!dat.is_null());
        //         let buf = unsafe { *(dat as *const [u32; 4]) };
        //         Some(buf)
        //     }
        //     None => None,
        // }

        let ewmh = events.atoms.ewmh_ptr();

        let mut error = null_mut();
        let mut extents: sys::xcb_ewmh_get_extents_reply_t = unsafe { zeroed() };

        let cookie = unsafe { sys::xcb_ewmh_get_frame_extents(ewmh, self.handle()) };
        let reply = unsafe {
            sys::xcb_ewmh_get_frame_extents_reply(
                ewmh,
                cookie,
                addr_of_mut!(extents),
                addr_of_mut!(error),
            )
        };
        assert!(XcbGenericError::new(error).is_none());
        assert_eq!(reply, 1);

        Some([extents.left, extents.right, extents.top, extents.bottom])
    }

    /// Gets the rectangle of the Inner-Content of the Window.
    pub fn content_rect(&self, events: &EventLoop) -> Rect {
        let geom = {
            let mut error = null_mut();
            let cookie = unsafe { sys::xcb_get_geometry(events.connection.handle, self.handle()) };
            let reply = unsafe {
                sys::xcb_get_geometry_reply(events.connection.handle, cookie, addr_of_mut!(error))
            };
            assert!(XcbGenericError::new(error).is_none());
            assert!(!reply.is_null());
            reply
        };
        let _geom_free = defer(|| unsafe {
            sys::free(geom as *mut c_void);
        });

        let tree = {
            let mut error = null_mut();
            let cookie = unsafe { sys::xcb_query_tree(events.connection.handle, self.handle()) };
            let reply = unsafe {
                sys::xcb_query_tree_reply(events.connection.handle, cookie, addr_of_mut!(error))
            };
            assert!(XcbGenericError::new(error).is_none());
            assert!(!reply.is_null());
            reply
        };
        let _tree_free = defer(|| unsafe {
            sys::free(tree as *mut c_void);
        });

        let trans = {
            let mut error = null_mut();
            let cookie = unsafe {
                sys::xcb_translate_coordinates(
                    events.connection.handle,
                    self.handle(),
                    (*tree).root,
                    (*geom).x,
                    (*geom).y,
                )
            };
            let reply = unsafe {
                sys::xcb_translate_coordinates_reply(
                    events.connection.handle,
                    cookie,
                    addr_of_mut!(error),
                )
            };
            assert!(XcbGenericError::new(error).is_none());
            assert!(!reply.is_null());
            reply
        };
        let _trans_free = defer(|| unsafe {
            sys::free(trans as *mut c_void);
        });

        unsafe {
            let x = (*trans).dst_x - (*geom).x;
            let y = (*trans).dst_y - (*geom).y;
            let w = (*geom).width;
            let h = (*geom).height;
            let _bw = (*geom).border_width;

            Rect::new(x as _, y as _, w as _, h as _)
        }
    }

    // ---------------------------------------------------------------- //

    /// Sets the rectangle of the Outer-Border of the Window.
    pub fn reposition_border(&self, events: &EventLoop, rect: Rect) {
        let mask = sys::XCB_CONFIG_WINDOW_X
            | sys::XCB_CONFIG_WINDOW_Y
            | sys::XCB_CONFIG_WINDOW_WIDTH
            | sys::XCB_CONFIG_WINDOW_HEIGHT;

        let mut list = [
            /* XCB_CONFIG_WINDOW_X */ rect.origin.x as i32,
            /* XCB_CONFIG_WINDOW_Y */ rect.origin.y as i32,
            /* XCB_CONFIG_WINDOW_WIDTH */ rect.size.w as i32,
            /* XCB_CONFIG_WINDOW_HEIGHT*/ rect.size.h as i32,
        ];

        if let Some(margins) = self.internal_margins(events) {
            list[2] -= (margins[0] + margins[1]) as i32;
            list[3] -= (margins[2] + margins[3]) as i32;
        }

        let cookie = unsafe {
            sys::xcb_configure_window_checked(
                events.connection.handle,
                self.handle(),
                mask as u16,
                void_of!(list),
            )
        };
        events.connection.check_cookie(cookie).unwrap();

        events.connection.flush();
    }

    /// Gets the rectangle of the Outer-Border of the Window.
    pub fn border_rect(&self, events: &EventLoop) -> Rect {
        let mut rect = self.content_rect(events);
        if let Some(margins) = self.internal_margins(events) {
            rect.origin.x -= margins[0] as Coord;
            rect.origin.y -= margins[2] as Coord;
            rect.size.w += (margins[0] + margins[1]) as Extent;
            rect.size.h += (margins[2] + margins[3]) as Extent;
        }
        rect
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Minimize - Maximize - Fullscreen - Restore
impl Window {
    // ---------------------------------------------------------------- //

    /// Minimizes the Window.
    pub fn minimize(&self, _events: &EventLoop) {
        todo!()
    }

    /// Returns whether or not the Window is Minimized.
    pub fn is_minimized(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //

    /// Maximizes the Window.
    pub fn maximize(&self, _events: &EventLoop) {
        todo!()
    }

    /// Returns whether or not the Window is Maximized.
    pub fn is_maximized(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //

    /// Fullscreens the Window.
    pub fn fullscreen(&self, _events: &EventLoop) {
        todo!()
    }

    /// Returns whether or not the Window is Fullscreened.
    pub fn is_fullscreen(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //

    /// Restores the Window from Minimized/Maximized/Fullscreen state.
    pub fn restore(&self, _events: &EventLoop) {
        todo!()
    }

    /// Returns whether or not the Window isn't Minimized, Maximized, or Fullscreened.
    pub fn is_normal(&self, events: &EventLoop) -> bool {
        !self.is_minimized(events) && !self.is_maximized(events) && !self.is_fullscreen(events)
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// The appearance and actions of a Window.
#[allow(unused)]
pub struct WindowType {
    /// The Window Style.
    pub style: WindowStyle,
    /// The Window Actions.
    pub actions: WindowActions,
}

/// The visual appearance of a Window.
pub enum WindowStyle {
    /// The Window has a Caption/Title-bar.
    Captioned,
    /// The Window has a thin frame.
    Bordered,
    /// The Window has no frame.
    Borderless,
}

/// The possible user-interactions on a Window.
pub struct WindowActions {
    /// Whether or not the user can move the window.
    pub movable: bool,
    /// Whether or not the user can close the window.
    pub closeable: bool,
    /// Whether or not the user can resize the window.
    pub resizable: bool,
}

// ---------------------------------------------------------------- //

/// Styles - Actions
impl Window {
    /// Sets the Type of a Window.
    pub fn set_type(&self, _events: &EventLoop, _wtype: WindowType) {
        todo!()
    }

    /// Gets the Type of a Window.
    pub fn get_type(&self, _events: &EventLoop) -> WindowType {
        todo!()
    }
}

// ================================================================================================================================ //
