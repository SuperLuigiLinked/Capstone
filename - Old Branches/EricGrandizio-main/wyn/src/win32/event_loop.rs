/*
 *  Crate: Wyn
 * Module: Win32 - Event Loop
 */

//! Functionality for Starting, Stopping, and Interacting with Event Loops.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::errors::*;
use super::event_data::EventData;
use super::events::EventHandler;
use super::inputs::*;
use super::types::*;
use crate::tasks::{ExecFuture, Task};

use std::sync::{Mutex, MutexGuard, TryLockError};

// ================================================================================================================================ //

/// Holds the state/context required to run a native Event/Message Loop.\
/// Only one `EventLoop` may exist at any given moment, and it may or may not be running.
/// ## Notes
/// On some platforms, it is required to run the `EventLoop` on the Main Thread.
pub struct EventLoop<'a> {
    /// The Event Handler that responds to events.
    pub(crate) handler: &'a dyn EventHandler,

    /// A list of callback functions awaiting to be executed.
    pub(crate) tasks: Mutex<Vec<Task>>,

    /// Platform-specific data necessary to run the Event Loop.
    pub(crate) data: EventData,

    /// Lock that guarantees Exclusive-Access to the Event Thread.
    _lock: MutexGuard<'a, ()>,
}

// [These trait implementations are implicit, but noted here for clarity]
// impl !Send for EventLoop<'_> {}
unsafe impl Sync for EventLoop<'_> {}

impl<'a> EventLoop<'a> {
    /// Constructs a new `EventLoop`, using the provided `EventHandler`.\
    /// ## PANICS
    /// * Panics if another `EventLoop` object exists and holds the Event Thread lock.
    /// * Panics if a previous `EventLoop` panicked while holding the Event Thread lock.
    pub fn new(handler: &'a dyn EventHandler) -> WinResult<Self> {
        let _lock = match EVENTLOOP_MUTEX.try_lock() {
            Ok(lock) => lock,
            Err(TryLockError::WouldBlock) => {
                panic!("Attempted to create multiple Event Loops simultaneously!")
            }
            Err(TryLockError::Poisoned(_)) => {
                panic!("Attempted to create a poisoned Event Loop after panic!")
            }
        };

        let tasks = Mutex::new(Vec::new());

        // SAFETY; The Event Thread lock is held.
        let data = unsafe { EventData::new()? };

        Ok(Self {
            handler,
            tasks,
            data,
            _lock,
        })
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Mutex to prevent creating multiple `EventLoop` objects at the same time.
static EVENTLOOP_MUTEX: Mutex<()> = Mutex::new(());

/// Reference to the currently running EventLoop.\
/// Lifetime is tied to an `EventLoopGuard` object in the `EventLoop::run` function.\
/// ## SAFETY
/// Should only ever be accessed by the Event Thread, and only while the EVENTLOOP_MUTEX is acquired by the current thread.\
/// Treat as Mutable in `EventLoopGuard` code, and Immutable elsewhere.
pub(crate) static mut EVENTLOOP: Option<&'static EventLoop<'static>> = None;

/// Drop-Guard for an `EventLoop`.\
/// Initializes the static `EventLoop` on creation, and Removes it when dropped.
struct EventLoopGuard<'a> {
    /// The actual `EventLoop` whose reference is temporarily placed at static scope.
    #[allow(unused)]
    events: &'a EventLoop<'a>,
}

impl<'a> EventLoopGuard<'a> {
    /// Constructs a new `EventLoopGuard`.
    /// ## PANICS
    /// * Panics if the Calling Thread is not the Event Thread.
    fn new(events: &'a EventLoop<'a>) -> Self {
        if !events.is_this_thread() {
            panic!("Attempted to run Event Loop while not on the Event Thread!");
        }

        // `transmute` in order to cast to `'static` lifetime.
        // SAFETY: This reference is tied to the lifetime of this object, and is removed when this object is dropped.
        let static_ref = unsafe { transmute(events) };

        // SAFETY: We are on the Event Thread and have Exclusive-Access (via the EventLoop's lock).
        let _ = unsafe { EVENTLOOP.insert(static_ref) };

        Self { events }
    }
}

impl<'a> Drop for EventLoopGuard<'a> {
    fn drop(&mut self) {
        self.events.data.set_running(false);

        // Close all Windows, now that Event Loop has stopped running.
        EventLoop::internal_close_thread_windows(self.events.data.thread_id);

        // SAFETY: We are on the Event Thread and have Exclusive-Access (via the EventLoop's lock).
        let _ = unsafe { EVENTLOOP.take() };
    }
}

// ================================================================================================================================ //

impl EventLoop<'_> {
    /// Starts a native Event/Message loop.
    /// ## PANICS
    /// * Panics if the Calling Thread is not the Event Thread.
    pub fn run(&self) {
        let guard = EventLoopGuard::new(self);

        self.handler.start(self);
        self.data.set_running(true);

        // SAFETY: This function is called while holding an `EventLoopGuard`.
        unsafe { self.internal_run() }

        self.handler.stop(self);
        drop(guard);
    }

    /// The internal native Event/Message Loop.
    /// ## SAFETY
    /// * Must be called on the Event Thread to function properly.
    /// * Must be called while holding an `EventLoopGuard` to ensure the validity of the static EVENTLOOP.
    unsafe fn internal_run(&self) {
        let _code = loop {
            errors::resume_if_panicking();

            // SAFETY: C Structs are safe to zero-initialize.
            let mut msg: sys::MSG = unsafe { zeroed() };

            // SAFETY: `lpmsg` is guaranteed to be a valid non-null pointer.
            let res = unsafe { sys::GetMessageW(addr_of_mut!(msg), 0, 0, 0) };

            errors::resume_if_panicking();

            // [DEBUG: Uncomment to see debug messages]
            //log::event("QUEUE", msg.hwnd, msg.message, msg.wParam, msg.lParam);

            match res {
                0 => break 0,
                -1 => break -1,
                _ => {
                    {
                        // SAFETY: `lpmsg` is guaranteed to be a valid non-null pointer.
                        let _r1 = unsafe { sys::TranslateMessage(addr_of!(msg)) };

                        // SAFETY: `lpmsg` is guaranteed to be a valid non-null pointer.
                        let _r2 = unsafe { sys::DispatchMessageW(addr_of!(msg)) };
                    }
                }
            }
        };
    }
}

// ================================================================================================================================ //

impl EventLoop<'_> {
    /// Returns a boolean indicating whether or not the Calling Thread is the Event Thread.
    pub fn is_this_thread(&self) -> bool {
        let thread_id = unsafe { sys::GetCurrentThreadId() };
        self.data.thread_id == thread_id
    }

    /// Returns a boolean indicating whether or not the Event Thread is running an Event/Message Loop.
    pub fn is_running(&self) -> bool {
        self.data.get_running()
    }

    /// If an Event/Message Loop is running, then request it to terminate.
    pub fn request_stop(&self) {
        let event_thread = self.data.thread_id;

        let callback = move || {
            // If any windows are in a Modal Loop (such as from dragging/resizing), then `PostQuitMessage` may be ignored.
            // As such, attempt to close every window to prevent that from causing issues.
            // (If the user wants to shut down, they're going to need to close all their windows anyways)
            EventLoop::internal_close_thread_windows(event_thread);

            // SAFETY: `PostQuitMessage` has no error cases, but it must be called on the Event Thread to work as intended.
            unsafe { sys::PostQuitMessage(0) };
        };

        self.execute_detached(callback);
    }

    /// Sleep the current thread until the Event Loop is running or it panicked.\
    /// Returns `false` if the Event Thread panicked.
    pub fn await_startup(&self) -> bool {
        /// Dummy Mutex for `CondVar` functions.
        static MUTEX: Mutex<()> = Mutex::new(());

        let guard = MUTEX.lock().unwrap();
        let _ = self.data.condvar.wait_while(guard, |_| {
            !errors::is_panicking() && !self.data.get_running()
        });

        !errors::is_panicking()
    }

    /// Sleep the current thread until the Event Loop is not running or it panicked.\
    /// Returns `false` if the Event Thread panicked.
    pub fn await_termination(&self) -> bool {
        /// Dummy Mutex for `CondVar` functions.
        static MUTEX: Mutex<()> = Mutex::new(());

        let guard = MUTEX.lock().unwrap();
        let _ = self.data.condvar.wait_while(guard, |_| {
            !errors::is_panicking() && self.data.get_running()
        });

        !errors::is_panicking()
    }
}

// ================================================================================================================================ //

/// Functionality relating to executing code on the Event Thread.
///
/// Sometimes, there are functions will only work if executed on the Main/Event Thread.
/// Normally, said thread is under control of the library, and user-code can only execute during `EventHandler` callbacks (which can be limiting).
/// These functions provide a way for users to temporarily take control of the Main/Event Thread
/// from a separate thread, allowing them to execute any code that must be run on said thread.
impl EventLoop<'_> {
    /// Executes the provided callback function on the Event Thread, and returns the result.
    pub fn execute<T, F>(&self, callback: F) -> ExecFuture<T>
    where
        T: Send + 'static,
        F: Send + 'static + FnOnce() -> T,
    {
        if self.is_this_thread() {
            let val = callback();
            ExecFuture::new_sync(Some(val))
        } else {
            let (fut_recv, fut_send) = ExecFuture::new_async(None);

            let task = Box::new(move || {
                let val = callback();
                fut_send.notify(val);
            });

            {
                let mut tasks = self.tasks.lock().unwrap();
                tasks.push(task);
            }

            self.signal_tasks();

            fut_recv
        }
    }

    /// Executes the provided callback function on the Event Thread, but discards the result.\
    /// Because the return value is discarded, this removes some limitations (such as requiring `T` to be `Send + 'static`).
    pub fn execute_discard<T, F>(&self, callback: F) -> ExecFuture<()>
    where
        F: Send + 'static + FnOnce() -> T,
    {
        if self.is_this_thread() {
            let _ = callback();
            ExecFuture::new_sync(Some(()))
        } else {
            let (fut_recv, fut_send) = ExecFuture::new_async(None);

            let task = Box::new(move || {
                let _ = callback();
                fut_send.notify(());
            });

            {
                let mut tasks = self.tasks.lock().unwrap();
                tasks.push(task);
            }

            self.signal_tasks();

            fut_recv
        }
    }

    /// Executes the provided callback function on the Event Thread, but discards the result.\
    /// Because the return value is discarded, this removes some limitations (such as requiring `T` to be `Send + 'static`).\
    /// By not synchronizing the Calling Thread with the Event Thread, a lot of synchronization overhead is removed.\
    /// This function will queue the callback function and immediately return, even if run on the Event Thread.
    pub fn execute_detached<T, F>(&self, callback: F)
    where
        F: Send + 'static + FnOnce() -> T,
    {
        let task = Box::new(move || {
            let _ = callback();
        });

        {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.push(task);
        }

        self.signal_tasks();
    }

    /// Signals to the Event Thread that there are Tasks that need executed.
    fn signal_tasks(&self) {
        let thread_id = self.data.thread_id;

        // SAFETY: The Win32-Error is checked after the call to this function.
        let res = sys_verify! { sys::PostThreadMessageW(thread_id, sys::WM_APP, 0, 0) };

        res.expect("EventLoop::Execute Signal-Tasks should not fail.");
    }

    /// If there are any Tasks waiting to be executed, remove them from the list and call them.
    fn clear_tasks(&self) {
        // Must be called from the Event Thread.
        assert!(self.is_this_thread());

        // Cannot hold the Tasks-lock while executing a task, otherwise a user-callback might cause a deadlock.
        // As such, the lock must be re-acquired/released for each task.
        let get_task = || {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.pop()
        };

        // Remove all tasks from the list.
        while let Some(task) = get_task() {
            task();
        }
    }
}

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

impl EventLoop<'_> {
    /// This is the Window Procedure (WNDPROC) for windows of the `"wyn"` window class.
    pub(crate) unsafe extern "system" fn internal_wndproc(
        hwnd: sys::HWND,
        umsg: sys::UINT,
        wparam: sys::WPARAM,
        lparam: sys::LPARAM,
    ) -> sys::LRESULT {
        // If no panics have occurred, attempt to dispatch to user-code.
        if !errors::internal_is_panicking() {
            let func = move || Self::internal_wndproc_impl(hwnd, umsg, wparam, lparam);

            match std::panic::catch_unwind(func) {
                Ok(res) => return res,
                Err(err) => errors::store_panic(err),
            }
        }
        // Otherwise, let the OS handle everything on its own.
        sys::DefWindowProcW(hwnd, umsg, wparam, lparam)
    }

    /// This is the part of the Window Procedure that attempts to dispatch OS events to the Wyn `EventHandler`.
    unsafe fn internal_wndproc_impl(
        hwnd: sys::HWND,
        umsg: sys::UINT,
        wparam: sys::WPARAM,
        lparam: sys::LPARAM,
    ) -> sys::LRESULT {
        // [DEBUG: Uncomment to see debug messages]
        //log::event("EVENT", hwnd, umsg, wparam, lparam);

        if let Some(events) = &EVENTLOOP {
            match umsg {
                // https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create
                sys::WM_CREATE => {
                    events.handler.window_open(events, hwnd);
                }
                // https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close
                sys::WM_CLOSE => {
                    events.handler.window_close(events, hwnd);
                }
                // https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd
                // sys::WM_ERASEBKGND => {
                //     eprintln!("ERASE!");
                //     events.handler.window_redraw(events, hwnd);
                //     eprintln!("ERASE-END!");
                //     return !0;
                // }
                // https://learn.microsoft.com/en-us/windows/win32/gdi/wm-paint
                sys::WM_PAINT => {
                    // SAFETY: C-Structs are safe to zero-initialize.
                    let mut ps = unsafe { zeroed() };

                    // SAFETY: <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint#return-value>
                    //          Result checked below. `EndPaint` guaranteed to be called at end of function.
                    let res = unsafe { sys::BeginPaint(hwnd, addr_of_mut!(ps)) };
                    assert_ne!(res, 0);
                    let _defer = defer(|| {
                        unsafe { sys::EndPaint(hwnd, addr_of_mut!(ps)) };
                    });

                    events.handler.window_redraw(events, hwnd);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate
                sys::WM_ACTIVATE => {
                    let focused = (wparam & 0xFFFF) != 0;
                    events.handler.window_focus(events, hwnd, focused);
                }
                // https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow
                sys::WM_SHOWWINDOW => {
                    let visible = wparam != 0;
                    events.handler.window_visibility(events, hwnd, visible);
                }
                // https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged
                sys::WM_WINDOWPOSCHANGED => {
                    if let Some(_wpos) = (lparam as *const sys::WINDOWPOS).as_ref() {
                        events.handler.window_reposition(events, hwnd);
                    }
                    return 0;
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove
                sys::WM_MOUSEMOVE => {
                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let point = Point::new(x as _, y as _);
                    events.handler.cursor_move(events, hwnd, point);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown
                sys::WM_LBUTTONDOWN => {
                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events
                        .handler
                        .button_press(events, hwnd, inputs::MB_LEFT, true)
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup
                sys::WM_LBUTTONUP => {
                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events.handler.button_press(events, hwnd, MB_LEFT, false)
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown
                sys::WM_MBUTTONDOWN => {
                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events.handler.button_press(events, hwnd, MB_MIDDLE, true)
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup
                sys::WM_MBUTTONUP => {
                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events.handler.button_press(events, hwnd, MB_MIDDLE, false)
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown
                sys::WM_RBUTTONDOWN => {
                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events.handler.button_press(events, hwnd, MB_RIGHT, true)
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup
                sys::WM_RBUTTONUP => {
                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events.handler.button_press(events, hwnd, MB_RIGHT, false)
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel
                sys::WM_MOUSEWHEEL => {
                    let _keys = sys::GET_KEYSTATE_WPARAM(wparam);

                    let delta = sys::GET_WHEEL_DELTA_WPARAM(wparam);
                    let norm_delta = (delta as ScrollDelta) / (sys::WHEEL_DELTA as ScrollDelta);

                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events.handler.scroll_wheel(events, hwnd, 0.0, norm_delta);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehwheel
                sys::WM_MOUSEHWHEEL => {
                    let _keys = sys::GET_KEYSTATE_WPARAM(wparam);

                    let delta = sys::GET_WHEEL_DELTA_WPARAM(wparam);
                    let norm_delta = (delta as ScrollDelta) / (sys::WHEEL_DELTA as ScrollDelta);

                    let x = sys::GET_X_LPARAM(lparam);
                    let y = sys::GET_Y_LPARAM(lparam);
                    let _point = Point::new(x as _, y as _);

                    events.handler.scroll_wheel(events, hwnd, norm_delta, 0.0);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown
                sys::WM_KEYDOWN => {
                    let keycode = KeyCode(wparam as sys::VIRTUAL_KEY);
                    events.handler.key_press(events, hwnd, keycode, true);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keyup
                sys::WM_KEYUP => {
                    let keycode = KeyCode(wparam as sys::VIRTUAL_KEY);
                    events.handler.key_press(events, hwnd, keycode, false);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown
                sys::WM_SYSKEYDOWN => {
                    let keycode = KeyCode(wparam as sys::VIRTUAL_KEY);
                    events.handler.key_press(events, hwnd, keycode, true);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup
                sys::WM_SYSKEYUP => {
                    let keycode = KeyCode(wparam as sys::VIRTUAL_KEY);
                    events.handler.key_press(events, hwnd, keycode, false);
                }
                // https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char
                sys::WM_CHAR => {
                    let code_points = [wparam as u16];
                    let mut decoder = char::decode_utf16(code_points.into_iter());

                    while let Some(Ok(chr)) = decoder.next() {
                        events.handler.character_input(events, hwnd, chr);
                    }
                }
                _ => {}
            }
        }
        sys::DefWindowProcW(hwnd, umsg, wparam, lparam)
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// This is a Windows Hook procedure to catch all messages.
    ///
    /// Under normal conditions, messages posted to threads (as opposed to windows) may be lost\
    /// if there is a window stuck in a modal loop (such as by dragging/resizing).
    ///
    /// Using this Windows Hook, we can intercept thread messages and respond to them before they are lost.
    pub(crate) unsafe extern "system" fn internal_hookproc(
        code: i32,
        wparam: sys::WPARAM,
        lparam: sys::LPARAM,
    ) -> sys::LRESULT {
        // If no panics have occurred, attempt to dispatch to user-code.
        if !errors::internal_is_panicking() {
            let func = move || Self::internal_hookproc_impl(code, wparam, lparam);

            match std::panic::catch_unwind(func) {
                Ok(res) => return res,
                Err(err) => errors::store_panic(err),
            }
        }
        // Otherwise, let the OS handle everything on its own.
        sys::CallNextHookEx(0, code, wparam, lparam)
    }

    /// This is the part of the Hook Procedure that reacts to Thread Messages.
    unsafe fn internal_hookproc_impl(
        code: i32,
        wparam: sys::WPARAM,
        lparam: sys::LPARAM,
    ) -> sys::LRESULT {
        // [DEBUG: Uncomment to see debug messages]
        //eprintln!("HOOK: ({code}) [{wparam}] <{lparam}>");

        let ptr = lparam as *const sys::MSG;

        if let Some(msg) = ptr.as_ref() {
            // [DEBUG: Uncomment to see debug messages]
            //log::event(" HOOK", msg.hwnd, msg.message, msg.wParam, msg.lParam);

            if msg.message == sys::WM_APP {
                if let Some(events) = EVENTLOOP {
                    events.clear_tasks();
                }
            }
        }

        sys::CallNextHookEx(0, code, wparam, lparam)
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Closes all windows on the Given Thread.
    /// All Errors are ignored.
    pub(crate) fn internal_close_thread_windows(thread_id: sys::DWORD) {
        let _res = unsafe { sys::EnumThreadWindows(thread_id, Some(Self::internal_close_proc), 0) };
    }

    /// The callback function given to the OS to close each individual window.
    /// All Errors are ignored.
    unsafe extern "system" fn internal_close_proc(
        hwnd: sys::HWND,
        _lparam: sys::LPARAM,
    ) -> sys::BOOL {
        let _res = sys::SendMessageW(hwnd, sys::WM_CLOSE, 0, 0);
        sys::TRUE
    }
}

// ================================================================================================================================ //
