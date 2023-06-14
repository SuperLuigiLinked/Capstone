/*
 *  Crate: Wyn
 * Module: Cocoa - Window
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::event_loop::EventLoop;
use super::types::Rect;

use std::borrow::Cow;

// ================================================================================================================================ //

/// Underlying OS Handle to a Window.
pub type WindowHandle = *mut sys::NSWindow;

/// Nonzero wrapper for Window Handles.
type NonZeroWindowHandle = NonNull<sys::NSWindow>;

/// Wrapper for Window objects.
#[repr(transparent)]
pub struct Window(NonZeroWindowHandle);

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

// ================================================================================================================================ //

impl Window {
    /// Returns the Native OS Handle to this Window.
    pub fn handle(&self) -> WindowHandle {
        self.0.as_ptr()
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
        let object = unsafe { sys::ns_window::alloc() };
        assert!(!object.is_null());

        let rect = sys::NSRect::new(sys::NSPoint::new(0.0, 0.0), sys::NSSize::new(640.0, 480.0));

        let style = sys::NSWindowStyleMask::NSTitledWindowMask
            | sys::NSWindowStyleMask::NSClosableWindowMask
            | sys::NSWindowStyleMask::NSMiniaturizableWindowMask
            | sys::NSWindowStyleMask::NSResizableWindowMask;

        let handle = unsafe {
            sys::ns_window::initWithContentRect_styleMask_backing_defer_(
                object,
                rect,
                style,
                sys::NSBackingStoreType::NSBackingStoreBuffered,
                sys::YES,
            )
        };

        let window = Window::try_from(handle).expect("Invalid Window Handle.");
        unsafe { sys::ns_window::center(handle) };

        unsafe { sys::ns_window::setDelegate_(handle, events.delegate) };
        unsafe { sys::ns_window::setContentView_(handle, events.delegate) };
        unsafe { sys::ns_window::setAcceptsMouseMovedEvents_(handle, sys::YES) };
        unsafe { sys::ns_window::setInitialFirstResponder_(handle, events.delegate) };
        let res = unsafe { sys::ns_window::makeFirstResponder_(handle, events.delegate) };
        assert_eq!(res, sys::YES);

        events.handler.window_open(events, handle);

        window
    }

    /// Returns whether or not the Window is Open.
    pub fn is_open(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //

    /// Closes the Window.
    pub fn close(self, events: &EventLoop) {
        unsafe { sys::ns_window::performClose_(self.handle(), events.delegate) }
    }

    /// Returns whether or not the Window is Closed.
    pub fn is_closed(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Focus - Show - Hide
impl Window {
    // ---------------------------------------------------------------- //

    /// Makes the Window visible and gives it focus.
    pub fn focus(&self, events: &EventLoop) {
        unsafe { sys::ns_window::makeKeyAndOrderFront_(self.handle(), events.delegate) };
    }

    /// Returns whether or not the Window is Focused.
    pub fn is_focused(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //

    /// Makes the Window visible.
    pub fn show(&self, events: &EventLoop) {
        unsafe { sys::ns_window::orderFront_(self.handle(), events.delegate) };
    }

    /// Returns whether or not the Window is Visible.
    pub fn is_visible(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //

    /// Makes the Window invisible.
    pub fn hide(&self, events: &EventLoop) {
        unsafe { sys::ns_window::orderOut_(self.handle(), events.delegate) };
    }

    /// Returns whether or not the Window is Hidden.
    pub fn is_hidden(&self, _events: &EventLoop) -> bool {
        todo!()
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Rename
impl Window {
    /// Sets the Name of the Window.
    pub fn rename(&self, _events: &EventLoop, name: &str) {
        let mut cow = Cow::Borrowed(name);
        if !cow.ends_with('\0') {
            cow.to_mut().push('\0');
        }
        assert!(cow.ends_with('\0'));

        unsafe {
            let nstr = sys::ns_string::stringWithUTF8String_(cow.as_ptr() as *const c_char);
            sys::ns_window::setTitle_(self.handle(), nstr);
        }
    }

    /// Gets the Name of the Window.
    pub fn name(&self, _events: &EventLoop) -> String {
        unsafe {
            let ns_str = sys::ns_window::title(self.handle());
            assert!(!ns_str.is_null());

            let cstr = sys::ns_string::UTF8String(ns_str);
            assert!(!cstr.is_null());
            let len = sys::strlen(cstr);

            let slice = std::slice::from_raw_parts(cstr as *const u8, len);
            String::from_utf8_lossy(slice).to_string()
        }
    }
}

// ================================================================================================================================ //

/// Reposition
impl Window {
    // ---------------------------------------------------------------- //

    /// Sets the rectangle of the Inner-Content of the Window.
    pub fn reposition_content(&self, _events: &EventLoop, rect: Rect) {
        let ns_rect = sys::NSRect::from(rect);
        let border = unsafe { sys::ns_window::frameRectForContentRect_(self.handle(), ns_rect) };
        unsafe { sys::ns_window::setFrame_display_(self.handle(), border, sys::YES) }
    }

    /// Gets the rectangle of the Inner-Content of the Window.
    pub fn content_rect(&self, _events: &EventLoop) -> Rect {
        let border = unsafe { sys::ns_window::frame(self.handle()) };
        let content = unsafe { sys::ns_window::contentRectForFrameRect_(self.handle(), border) };
        Rect::from(content)
    }

    // ---------------------------------------------------------------- //

    /// Sets the rectangle of the Outer-Border of the Window.
    pub fn reposition_border(&self, _events: &EventLoop, rect: Rect) {
        let ns_rect = rect.into();
        unsafe { sys::ns_window::setFrame_display_(self.handle(), ns_rect, sys::YES) }
    }

    /// Gets the rectangle of the Outer-Border of the Window.
    pub fn border_rect(&self, _events: &EventLoop) -> Rect {
        let ns_rect = unsafe { sys::ns_window::frame(self.handle()) };
        Rect::from(ns_rect)
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
