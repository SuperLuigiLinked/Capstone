/*
 *  Crate: Wyn
 * Module: Win32 - Window
 */

//! Functionality pertaining to manipulating windows on the desktop.
//!
//! Once an `EventLoop` has been started, it can be used to open new windows.
//! These windows can then be freely manipulated (such as position, size, title, appearance...) by the program.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::errors::*;
use super::event_loop::EventLoop;
use super::screen::Screen;
use super::types::{NativeRect, Rect};

// ================================================================================================================================ //

/// Underlying OS Handle to a Window.
pub type WindowHandle = sys::HWND;

/// Nonzero wrapper for Window Handles.
type NonZeroWindowHandle = NonZero<sys::HWND>;

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

    /// Opens a new Window.
    pub fn open(events: &EventLoop) -> WinResult<Window> {
        // SAFETY: Must be called on the Event Thread.
        let fut = events.execute(move || unsafe {
            let events = event_loop::EVENTLOOP.unwrap();
            Self::internal_open(events)
        });
        fut.wait()
    }

    /// Returns whether or not the Window is Open.
    pub fn is_open(&self, _events: &EventLoop) -> bool {
        // SAFETY: This function has no error conditions.
        unsafe { sys::IsWindow(self.handle()) != sys::FALSE }
    }

    /// Internal function for opening windows.
    /// ## SAFETY
    /// Must be called on the Event Thread.
    unsafe fn internal_open(events: &EventLoop) -> WinResult<Window> {
        let res = sys_verify! {
            sys::CreateWindowExW(
                Self::WS_EX_DEFAULT,
                event_data::WCLASS_NAME,
                sys::w!(""),
                sys::WS_OVERLAPPEDWINDOW,
                sys::CW_USEDEFAULT,
                sys::CW_USEDEFAULT,
                sys::CW_USEDEFAULT,
                sys::CW_USEDEFAULT,
                0,
                0,
                events.data.class.hinstance,
                null_mut(),
            )
        };

        // <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw#return-value>
        let handle = res?;

        let is_layered = (Self::WS_EX_DEFAULT & sys::WS_EX_LAYERED) == sys::WS_EX_LAYERED;

        if is_layered && (handle != 0) {
            let _res =
                sys_verify! { sys::SetLayeredWindowAttributes(handle, 0, 0xFF, sys::LWA_ALPHA) };
            let _ = _res.unwrap();
        }

        // let dwm_blur = sys::DWM_BLURBEHIND {
        //     dwFlags: sys::DWM_BB_ENABLE,
        //     fEnable: sys::TRUE,
        //     hRgnBlur: 0,
        //     fTransitionOnMaximized: 0,
        // };
        // let res = sys::DwmEnableBlurBehindWindow(handle, &dwm_blur);
        // assert_eq!(res, sys::S_OK);

        Ok(Window::try_from(handle).unwrap())
    }

    // ---------------------------------------------------------------- //

    /// Closes the Window.
    pub fn close(self, events: &EventLoop) -> WinResult<()> {
        // SAFETY: Must be called on the Event Thread.
        let fut = events.execute(move || unsafe {
            let events = event_loop::EVENTLOOP.unwrap();
            Self::internal_close(self, events)
        });
        fut.wait()
    }

    /// Internal function for closing windows.
    /// ## SAFETY
    /// Must be called on the Event Thread.
    unsafe fn internal_close(self, _events: &EventLoop) -> WinResult<()> {
        // SAFETY: The result of this call is checked by `sys_verify` below.
        let res = sys_verify! { sys::SendMessageW(self.handle(), sys::WM_CLOSE, 0, 0) };
        res.map(|_| ())
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Focus - Show - Hide
impl Window {
    // ---------------------------------------------------------------- //

    /// Makes the Window visible and gives it focus.
    pub fn focus(&self, _events: &EventLoop) -> WinResult<()> {
        // SAFETY: This function does not indicate any error conditions.
        unsafe { sys::ShowWindow(self.handle(), sys::SW_SHOW) };

        // SAFETY: This function does not set the Win32 Thread Error.
        let _res = unsafe { sys::SetForegroundWindow(self.handle()) };
        //assert_ne!(_res, 0);

        Ok(())
    }

    /// Returns whether or not the Window is Focused.
    pub fn is_focused(&self, _events: &EventLoop) -> bool {
        // SAFETY: This function does not indicate any error conditions.
        self.handle() == unsafe { sys::GetForegroundWindow() }
    }

    // ---------------------------------------------------------------- //

    /// Makes the Window visible.
    pub fn show(&self, _events: &EventLoop) -> WinResult<()> {
        // SAFETY: This function does not indicate any error conditions.
        unsafe { sys::ShowWindow(self.handle(), sys::SW_SHOWNA) };
        Ok(())
    }

    /// Returns whether or not the Window is Visible.
    pub fn is_visible(&self, _events: &EventLoop) -> bool {
        // SAFETY: This function does not indicate any error conditions.
        sys::FALSE != unsafe { sys::IsWindowVisible(self.handle()) }
    }

    // ---------------------------------------------------------------- //

    /// Makes the Window invisible.
    pub fn hide(&self, _events: &EventLoop) -> WinResult<()> {
        // SAFETY: This function does not indicate any error conditions.
        unsafe { sys::ShowWindow(self.handle(), sys::SW_HIDE) };
        Ok(())
    }

    /// Returns whether or not the Window is Hidden.
    pub fn is_hidden(&self, _events: &EventLoop) -> bool {
        // SAFETY: This function does not indicate any error conditions.
        sys::FALSE == unsafe { sys::IsWindowVisible(self.handle()) }
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Minimize - Maximize - Fullscreen - Restore
impl Window {
    // ---------------------------------------------------------------- //

    /// Minimizes the Window.
    pub fn minimize(&self, _events: &EventLoop) -> WinResult<()> {
        // SAFETY: This function does not indicate any error conditions.
        unsafe { sys::ShowWindow(self.handle(), sys::SW_MINIMIZE) };
        Ok(())
    }

    /// Returns whether or not the Window is Minimized.
    pub fn is_minimized(&self, _events: &EventLoop) -> bool {
        // SAFETY: This function does not indicate any error conditions.
        sys::FALSE != unsafe { sys::IsIconic(self.handle()) }
    }

    // ---------------------------------------------------------------- //

    /// Maximizes the Window.
    pub fn maximize(&self, _events: &EventLoop) -> WinResult<()> {
        // SAFETY: This function does not indicate any error conditions.
        unsafe { sys::ShowWindow(self.handle(), sys::SW_MAXIMIZE) };
        Ok(())
    }

    /// Returns whether or not the Window is Maximized.
    pub fn is_maximized(&self, _events: &EventLoop) -> bool {
        // SAFETY: This function does not indicate any error conditions.
        sys::FALSE != unsafe { sys::IsZoomed(self.handle()) }
    }

    // ---------------------------------------------------------------- //

    /// Fullscreens the Window. (Borderless Fullscreen)
    pub fn fullscreen(&self, events: &EventLoop) -> WinResult<()> {
        self.set_style(events, WindowStyle::Borderless)?;
        self.maximize(events)?;
        Ok(())
    }

    /// Returns whether or not the Window is Fullscreened.
    pub fn is_fullscreen(&self, events: &EventLoop) -> bool {
        if let Ok(style) = self.style(events) {
            (style == WindowStyle::Borderless) && self.is_maximized(events)
        } else {
            false
        }
    }

    // ---------------------------------------------------------------- //

    /// Restores the Window from Minimized/Maximized/Fullscreen state.
    pub fn restore(&self, _events: &EventLoop) -> WinResult<()> {
        // SAFETY: This function does not indicate any error conditions.
        unsafe { sys::ShowWindow(self.handle(), sys::SW_RESTORE) };
        Ok(())
    }

    /// Returns whether or not the Window isn't Minimized, Maximized, or Fullscreened.
    pub fn is_normal(&self, events: &EventLoop) -> bool {
        !self.is_minimized(events) && !self.is_maximized(events) && !self.is_fullscreen(events)
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Rename
impl Window {
    /// Sets the Name of the Window.
    pub fn rename(&self, _events: &EventLoop, name: &str) -> WinResult<()> {
        // Convert the text into UTF-16 Code Points.
        let os_str = OsStr::new(name);
        let encoder = os_str.encode_wide();
        let mut buf: Vec<u16> = encoder.collect();

        // Add the null-terminator.
        buf.push(0);

        // Attempt to set the title.
        // SAFETY: The buffer is guaranteed to be null-terminated.
        let res = sys_verify! { sys::SetWindowTextW(self.handle(), buf.as_ptr()) };
        res.map(|_| ())
    }

    /// Gets the Name of the Window.
    pub fn name(&self, _events: &EventLoop) -> WinResult<String> {
        let hwnd = self.handle();

        // Get the length of the text string, including the null-terminator.
        // SAFETY: Returned length is checked by `sys_verify` below.
        let res = sys_verify! { sys::GetWindowTextLengthW(hwnd) };
        let nt_len = (res?) + 1;

        // Allocate and write the contents of the text string.
        let mut os_buf = vec![0; nt_len as usize];
        // SAFETY: The buffer is valid, the `len` includes the null-terminator (as required).
        let res = sys_verify! { sys::GetWindowTextW(hwnd, os_buf.as_mut_ptr(), nt_len) };
        let written_len = res?;
        assert_eq!(nt_len - 1, written_len);

        // Convert the text string into a `String`.
        let os_text = OsString::from_wide(&os_buf);
        let utf8_text = os_text.to_string_lossy();
        let text = utf8_text.trim_end_matches('\0').to_owned();

        Ok(text)
    }
}

// ================================================================================================================================ //

/// Reposition
impl Window {
    // ---------------------------------------------------------------- //

    /// Sets the rectangle of the Inner-Content of the Window.
    pub fn reposition_content(&self, _events: &EventLoop, rect: Rect) -> WinResult<()> {
        let flags = sys::SWP_NOZORDER | sys::SWP_NOOWNERZORDER | sys::SWP_NOACTIVATE;
        let nt_rc = NativeRect::from(rect);
        self.internal_reposition(nt_rc, true, flags)
    }

    /// Sets the rectangle of the Outer-Border of the Window.
    pub fn reposition_border(&self, _events: &EventLoop, rect: Rect) -> WinResult<()> {
        let flags = sys::SWP_NOZORDER | sys::SWP_NOOWNERZORDER | sys::SWP_NOACTIVATE;
        let nt_rc = NativeRect::from(rect);
        self.internal_reposition(nt_rc, false, flags)
    }

    // ---------------------------------------------------------------- //

    /// Gets the rectangle of the Inner-Content of the Window.
    pub fn content_rect(&self, _events: &EventLoop) -> WinResult<Rect> {
        // SAFETY: C-Structs are safe to zero-initialize.
        let mut nt_pt = unsafe { zeroed() };
        // SAFETY: This function does not set the WIN32-ERROR Code.
        let res = unsafe { sys::ClientToScreen(self.handle(), addr_of_mut!(nt_pt)) };
        assert_ne!(res, 0);

        // SAFETY: C-Structs are safe to zero-initialize.
        let mut nt_rc = unsafe { zeroed() };
        // SAFETY: This function's result-code is checked below by `sys_verify`.
        let res = sys_verify! { sys::GetClientRect(self.handle(), addr_of_mut!(nt_rc)) };
        let _ = res?;

        // Offset the returned rectangle [with origin (0,0)] by the window's origin point.
        nt_rc.left += nt_pt.x;
        nt_rc.right += nt_pt.x;
        nt_rc.top += nt_pt.y;
        nt_rc.bottom += nt_pt.y;

        let rect = Rect::from(nt_rc);
        Ok(rect)
    }

    /// Gets the rectangle of the Outer-Border of the Window.
    pub fn border_rect(&self, _events: &EventLoop) -> WinResult<Rect> {
        // SAFETY: C-Structs are safe to zero-initialize.
        let mut nt_rc = unsafe { zeroed() };
        // SAFETY: This function's result-code is checked below by `sys_verify`.
        let res = sys_verify! { sys::GetWindowRect(self.handle(), addr_of_mut!(nt_rc)) };
        let _ = res?;

        let rect = Rect::from(nt_rc);
        Ok(rect)
    }

    // ---------------------------------------------------------------- //

    /// Repositions a window, adjust the rectangle as needed.
    fn internal_reposition(
        &self,
        mut rect: NativeRect,
        adjust: bool,
        flags: sys::SET_WINDOW_POS_FLAGS,
    ) -> WinResult<()> {
        if adjust {
            let NativeStyle { ws_style, ex_style } = self.internal_get_style()?;

            // SAFETY: This function does not set the WIN32-ERROR Code.
            let dpi = unsafe { sys::GetDpiForWindow(self.handle()) };
            assert_ne!(dpi, 0);

            // SAFETY: This function's result-code is checked below by `sys_verify`.
            let res = sys_verify! {
                sys::AdjustWindowRectExForDpi(
                    addr_of_mut!(rect),
                    ws_style,
                    sys::FALSE,
                    ex_style,
                    dpi,
                )
            };
            let _ = res?;
        }

        // SAFETY: This function's result-code is checked below by `sys_verify`.
        let res = sys_verify! {
            sys::SetWindowPos(
                self.handle(),
                0,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                flags,
            )
        };
        res.map(|_| ())
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// The visual appearance of a Window.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowStyle {
    /// The Window has a Titlebar.
    Captioned,
    /// The Window has a Thin Frame.
    Bordered,
    /// The Window has No Frame.
    Borderless,
}

/// The Native OS styles for windows.
struct NativeStyle {
    /// The Regular Window-Style.
    ws_style: sys::WINDOW_STYLE,
    /// The Extended Window-Style.
    ex_style: sys::WINDOW_EX_STYLE,
}

// ---------------------------------------------------------------- //

/// Styles - Actions
impl Window {
    /// Default Win32 Extended-Windows Style.
    const WS_EX_DEFAULT: sys::WINDOW_EX_STYLE =
        sys::WS_EX_APPWINDOW | sys::WS_EX_NOREDIRECTIONBITMAP;

    // ---------------------------------------------------------------- //

    /// Sets the Type of a Window.
    pub fn set_style(&self, events: &EventLoop, style: WindowStyle) -> WinResult<()> {
        // Maintain the current visibility.
        let vis_flag: sys::WINDOW_STYLE = if self.is_visible(events) {
            sys::WS_VISIBLE
        } else {
            0
        };

        let nt_style = match style {
            WindowStyle::Captioned => NativeStyle {
                ws_style: sys::WS_OVERLAPPEDWINDOW | vis_flag,
                ex_style: Self::WS_EX_DEFAULT,
            },
            WindowStyle::Bordered => NativeStyle {
                ws_style: sys::WS_OVERLAPPED | sys::WS_BORDER | vis_flag,
                ex_style: Self::WS_EX_DEFAULT,
            },
            WindowStyle::Borderless => NativeStyle {
                ws_style: sys::WS_POPUP | vis_flag,
                ex_style: Self::WS_EX_DEFAULT,
            },
        };

        self.internal_set_style(nt_style)
    }

    /// Gets the Type of a Window.
    pub fn style(&self, _events: &EventLoop) -> WinResult<WindowStyle> {
        let nt_style = self.internal_get_style()?;

        let has_caption = (nt_style.ws_style & sys::WS_CAPTION) == sys::WS_CAPTION;
        let has_border = (nt_style.ws_style & sys::WS_BORDER) == sys::WS_BORDER;

        if has_caption {
            Ok(WindowStyle::Captioned)
        } else if has_border {
            Ok(WindowStyle::Bordered)
        } else {
            Ok(WindowStyle::Borderless)
        }
    }

    // ---------------------------------------------------------------- //

    /// Internal convenience-function to get both Win32 window styles.
    fn internal_get_style(&self) -> WinResult<NativeStyle> {
        // SAFETY: This function's result-code is checked below by `sys_verify`.
        let ws_res = sys_verify! { sys::GetWindowLongPtrW(self.handle(), sys::GWL_STYLE) };
        let ws_style = (ws_res?) as sys::WINDOW_STYLE;

        // SAFETY: This function's result-code is checked below by `sys_verify`.
        let ex_res = sys_verify! { sys::GetWindowLongPtrW(self.handle(), sys::GWL_EXSTYLE) };
        let ex_style = (ex_res)? as sys::WINDOW_EX_STYLE;

        let style = NativeStyle { ws_style, ex_style };
        Ok(style)
    }

    /// Internal convenience-function to set both Win32 window styles.
    fn internal_set_style(&self, style: NativeStyle) -> WinResult<()> {
        let ws_style = style.ws_style as sys::LONG_PTR;
        // SAFETY: This function's result-code is checked below by `sys_verify`.
        let ws_res = sys_verify! {{
            WinError::clear();
            sys::SetWindowLongPtrW(self.handle(), sys::GWL_STYLE, ws_style)
        }};
        let _ = ws_res?;

        let ex_style = style.ex_style as sys::LONG_PTR;
        // SAFETY: This function's result-code is checked below by `sys_verify`.
        let ex_res = sys_verify! {{
            WinError::clear();
            sys::SetWindowLongPtrW(self.handle(), sys::GWL_EXSTYLE, ex_style)
        }};
        let _ = ex_res?;

        Ok(())
    }

    // ---------------------------------------------------------------- //
}

// ================================================================================================================================ //

/// Screens
impl Window {
    /// Returns the the closest Screen the Window is currently occupying.
    pub fn screen(&self) -> Screen {
        // SAFETY: All the arguments are guaranteed to be valid.
        let handle =
            unsafe { sys::MonitorFromWindow(self.handle(), sys::MONITOR_DEFAULTTONEAREST) };

        Screen::new(handle).expect("The Nearest Screen should always exist.")
    }
}

// ================================================================================================================================ //

/// Redraw
impl Window {
    /// Requests the Window to redraw its contents.
    pub fn request_redraw(&self, events: &EventLoop) {
        let handle = self.handle();

        let redraw = move || {
            let flags = //sys::RDW_UPDATENOW
                sys::RDW_INTERNALPAINT
                | sys::RDW_INVALIDATE
                | sys::RDW_ERASE
                //| sys::RDW_ERASENOW | sys::RDW_UPDATENOW
                | sys::RDW_ALLCHILDREN;

            // SAFETY: NULL is acceptable. This function does not set the Win32 Error Code.
            let res = unsafe { sys::RedrawWindow(handle, null(), 0, flags) };

            // <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#return-value>
            if res == 0 {
                Err(())
            } else {
                Ok(())
            }
        };

        events.execute_detached(redraw);
    }
}

// ================================================================================================================================ //
