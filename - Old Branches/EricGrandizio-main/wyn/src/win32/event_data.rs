/*
 *  Crate: Wyn
 * Module: Win32 - Event Data
 */

//! Platform-specific types to aid running an Event Loop.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::errors::*;
use super::event_loop::*;

use std::sync::atomic::AtomicBool;
use std::sync::Condvar;

// ================================================================================================================================ //

/// Data stored within an `EventLoop` object.
pub struct EventData {
    /// Flag to indicate whether the Event Loop is running or not.
    pub(crate) running: AtomicBool,

    /// Condion Variable for other threads to await Event Loop startup.
    pub(crate) condvar: Condvar,

    /// The native OS Thread-ID for the Event Thread.
    pub(crate) thread_id: sys::DWORD,

    /// Win32 Hook Handle.
    #[allow(unused)]
    pub(crate) hook: WinHook,

    /// Win32 Window Class.
    #[allow(unused)]
    pub(crate) class: WinClass,
}

impl EventData {
    /// Constructs a new EventData object.
    /// ## SAFETY
    /// This function must be called during the construction of an EventLoop, while the Event Thread lock is held.
    pub unsafe fn new() -> WinResult<Self> {
        let running = AtomicBool::new(false);

        let condvar = Condvar::new();

        // SAFETY: This function cannot fail.
        let thread_id = unsafe { sys::GetCurrentThreadId() };

        let hook = WinHook::new(thread_id)?;

        let class = WinClass::new()?;

        Ok(Self {
            running,
            condvar,
            thread_id,
            hook,
            class,
        })
    }

    /// Atomically load Running-Flag.
    pub(crate) fn get_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Acquire)
    }

    /// Atomically set Running-Flag.
    pub(crate) fn set_running(&self, status: bool) {
        self.running
            .store(status, std::sync::atomic::Ordering::Release);
        self.condvar.notify_all();
    }
}

// ================================================================================================================================ //

/// The Name of the Win32 Window Class.
pub(crate) const WCLASS_NAME: *const u16 = sys::w!("wyn");

/// Win32 Window Class wrapper.
pub(crate) struct WinClass {
    /// Handle to the Win32 Module Instance.
    pub hinstance: sys::HINSTANCE,

    /// Atom representing the Window Class.
    #[allow(unused)]
    pub atom: sys::ATOM,
}

impl WinClass {
    /// Registers a new Window Class.
    pub fn new() -> WinResult<Self> {
        // SAFETY: NULL is an acceptable parameter.
        let res = sys_verify! { sys::GetModuleHandleW(null()) };
        let hinstance = res?;

        let res = sys_verify! { sys::LoadCursorW(0, sys::IDC_ARROW) };
        let cursor = res?;

        // Prepare window class so windows can be created.
        // <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw>
        let class = sys::WNDCLASSEXW {
            cbSize: size_of::<sys::WNDCLASSEXW>() as u32,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: 0,
            hIconSm: 0,
            hCursor: cursor,
            //hbrBackground: 0,
            hbrBackground: (sys::COLOR_WINDOW + 1) as sys::HBRUSH,
            hInstance: hinstance,
            lpszMenuName: null(),
            lpszClassName: sys::w!("wyn"),
            lpfnWndProc: Some(EventLoop::internal_wndproc),
            style: sys::CS_HREDRAW | sys::CS_VREDRAW,
        };

        // Attempt to register the window class.
        let res = sys_verify! { sys::RegisterClassExW(addr_of!(class)) };
        let atom = res?;

        Ok(Self { hinstance, atom })
    }
}

impl Drop for WinClass {
    fn drop(&mut self) {
        let res = sys_verify! { sys::UnregisterClassW(WCLASS_NAME, self.hinstance) };
        res.expect("Win32 WNDCLASS Un-Registration should not fail.");
    }
}

// ================================================================================================================================ //

/// Win32 Hook wrapper.
pub(crate) struct WinHook(pub sys::HHOOK);

impl WinHook {
    /// Registers a new Hook.
    fn new(thread_id: sys::DWORD) -> WinResult<Self> {
        let res = sys_verify! {
            sys::SetWindowsHookExW(sys::WH_GETMESSAGE, Some(EventLoop::internal_hookproc), 0, thread_id)
        };
        let hook = res?;

        Ok(Self(hook))
    }
}

impl Drop for WinHook {
    fn drop(&mut self) {
        let res = sys_verify! { sys::UnhookWindowsHookEx(self.0) };
        res.expect("Win32 HOOK Un-Registration should not fail.");
    }
}

// ================================================================================================================================ //
