/*
 *  Crate: Wyn
 * Module: Win32 - Screen
 */

//! Functionality for querying the state of Monitors/Screens and the Desktop environment.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::event_loop::EventLoop;
use super::types::Rect;

// ================================================================================================================================ //

/// Native OS Representation for Screens.
pub type NativeScreen = sys::HMONITOR;

/// Nonzero wrapper for Native Screens.
type NonzeroScreen = NonZero<NativeScreen>;

/// Wrapper type for Screen Handles.
pub struct Screen(NonzeroScreen);

/// Holds all information about a Screen.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct ScreenInfo {
    /// The Bounding-Rectangle.
    pub rect: Rect,

    /// The Textual Name.
    pub name: String,
}

// ================================================================================================================================ //

impl Screen {
    /// Returns the bounding-rectangle of the Screen.
    pub fn rect(&self, _events: &EventLoop) -> Rect {
        let info = Self::internal_monitor_info(self.0.get()).unwrap();

        let win_rect = info.monitorInfo.rcMonitor;
        Rect::from(win_rect)
    }

    /// Returns the textual-name of the Screen.
    pub fn name(&self, _events: &EventLoop) -> String {
        let info = Self::internal_monitor_info(self.0.get()).unwrap();

        let os_buf = info.szDevice;
        let os_text = OsString::from_wide(&os_buf);
        let utf8_text = os_text.to_string_lossy();
        let trimmed = utf8_text.trim_end_matches('\0');
        trimmed.to_owned()
    }

    /// Queries all the Information about the Screen.
    pub fn info(&self, events: &EventLoop) -> ScreenInfo {
        ScreenInfo {
            rect: self.rect(events),
            name: self.name(events),
        }
    }
}

// ================================================================================================================================ //

impl Screen {
    /// Creates a new Screen object with the given handle.
    pub(crate) fn new(handle: NativeScreen) -> Option<Screen> {
        NonzeroScreen::new(handle).map(Self)
    }

    /// Returns the Primary Screen.
    pub fn primary(_events: &EventLoop) -> Self {
        let point = sys::POINT { x: 0, y: 0 };
        // SAFETY: All the arguments are guaranteed to be valid.
        let handle = unsafe { sys::MonitorFromPoint(point, sys::MONITOR_DEFAULTTOPRIMARY) };

        Screen::new(handle).expect("The Primary Screen should always exist.")
    }

    /// Collects a list of all the Screens.
    pub fn collect(_events: &EventLoop) -> Vec<Self> {
        let mut screens = Vec::new();

        let data = addr_of_mut!(screens) as sys::LPARAM;
        // SAFETY: `data` is a `*mut Vec<Screen>` cast to an `LPARAM`.
        unsafe {
            sys::EnumDisplayMonitors(0, null(), Some(Self::internal_monitor_proc), data);
        };

        screens
    }
}

// ================================================================================================================================ //

/// Windows-exclusive functionality.
impl Screen {
    /// The internal Monitor-Enumeration Procedure that reads `MonitorInfo` and pushes it into the provided `Vec`.
    /// ## SAFETY
    /// `data` must be a `*mut Vec<Screen>` cast to an `LPARAM`.
    extern "system" fn internal_monitor_proc(
        handle: sys::HMONITOR,
        _hdc: sys::HDC,
        _lprect: *mut sys::RECT,
        data: sys::LPARAM,
    ) -> sys::BOOL {
        if let Some(screens) = unsafe { (data as *mut Vec<Screen>).as_mut() } {
            if let Some(screen) = Screen::new(handle) {
                screens.push(screen);
            }

            // Continue iteration.
            sys::TRUE
        } else {
            // Stop iteration.
            sys::FALSE
        }
    }

    /// Internal function that retrieves the info associated with the Screen.
    fn internal_monitor_info(handle: sys::HMONITOR) -> Result<sys::MONITORINFOEXW, ()> {
        // SAFETY: C structs are safe to zero-initialize.
        let mut info: sys::MONITORINFOEXW = unsafe { zeroed() };
        info.monitorInfo.cbSize = size_of::<sys::MONITORINFOEXW>() as u32;

        // SAFETY: The `.cbSize` member was set above, the `ptr` is guaranteed to be valid, and the return value is checked.
        let res = unsafe { sys::GetMonitorInfoW(handle, addr_of_mut!(info.monitorInfo)) };

        if res == 0 {
            Err(())
        } else {
            Ok(info)
        }
    }
}

// ================================================================================================================================ //
