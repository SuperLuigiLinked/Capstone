/*
 *  Crate: Wyn
 * Module: X11 - Event Loop
 */

//! ...

// ================================================================================================================================ //

use crate::inputs::KeyCode;

#[allow(unused_imports)]
use super::*;

use super::errors::*;
use super::events::EventHandler;
use super::inputs::*;
use super::types::*;

use std::cell::UnsafeCell;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::sync::{Mutex, MutexGuard, TryLockError};

// ================================================================================================================================ //

/// Holds the state/context required to run a native Event/Message Loop.\
/// Only one `EventLoop` may exist at any given moment, and it may or may not be running.
/// ## Notes
/// On some platforms, it is required to run the `EventLoop` on the Main Thread.
pub struct EventLoop<'a> {
    /// The Event Handler that responds to events.
    pub(crate) handler: &'a dyn EventHandler,

    /// List of available Atoms.
    pub(crate) atoms: XcbAtomList,

    /// Allows signalling from other threads while waiting on events.
    pub(crate) epoller: Epoller,

    /// XCB Connection.
    pub(crate) connection: XcbConnection,

    #[allow(unused)]
    /// Lock that guarantees Exclusive-Access to the Event Thread.
    lock: MutexGuard<'a, ()>,
}

// [These trait implementations are implicit, but noted here for clarity]
// impl !Send for EventLoop<'_> {}
unsafe impl Sync for EventLoop<'_> {}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Mutex to prevent creating multiple `EventLoop` objects at the same time.
static EVENTLOOP_MUTEX: Mutex<()> = Mutex::new(());

/// Reference to the currently running EventLoop.\
/// Lifetime is tied to an `EventLoopGuard` object in the `EventLoop::run` function.\
/// ## SAFETY
/// Should only ever be accessed by the Event Thread, and only while the EVENTLOOP_MUTEX is acquired by the current thread.\
/// Treat as Mutable in `EventLoopGuard` code, and Immutable elsewhere.
static mut EVENTLOOP: Option<&'static EventLoop<'static>> = None;

/// Drop-Guard for an `EventLoop`.\
/// Initializes the static `EventLoop` on creation, and Removes it when dropped.
struct EventLoopGuard<'a> {
    /// The actual `EventLoop` whose reference is temporarily placed at static scope.
    events: &'a EventLoop<'a>,
}

impl<'a> EventLoopGuard<'a> {
    /// Constructs a new `EventLoopGuard`.
    /// ## PANICS
    /// * Panics if the Calling Thread is not the Event Thread.
    fn new(events: &'a EventLoop<'a>) -> Self {
        // if !events.is_this_thread() {
        //     panic!("Attempted to run Event Loop while not on the Event Thread!");
        // }

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
        // SAFETY: We are on the Event Thread and have Exclusive-Access (via the EventLoop's lock).
        let _ = unsafe { EVENTLOOP.take() };
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// The `panic` that was caught to cross FFI-boundaries.\
/// Lifetime is tied to an `EventLoopGuard` object in the `EventLoop::run` function.\
/// ## SAFETY
/// Should only ever be accessed by the Event Thread, and only while the EVENTLOOP_MUTEX is acquired by the current thread.
static mut PANIC: Option<BoxedPanic> = None;

/// Stores a `panic` to carry across FFI-boundaries.\
/// If a second `panic` is attempted to be stored while a previous one is stored, the process will abort.
/// ## SAFETY
/// Should only ever be called by the Event Thread, and only while the EVENTLOOP_MUTEX is acquired by the current thread.
#[allow(unused)]
unsafe fn store_panic(err: BoxedPanic) {
    match &mut PANIC {
        // There was not a previous `panic`, so store this one and tell the Event Loop to shut down.
        None => {
            let _ = PANIC.insert(err);
            if let Some(events) = EVENTLOOP {
                events.request_stop();
            }
        }
        // There was already a previous `panic`, so abort the process.
        Some(_err) => std::process::abort(),
    };
}

/// Resumes a `panic` that was caught earlier, if there is one.
/// ## SAFETY
/// Should only ever be called by the Event Thread, and only while the EVENTLOOP_MUTEX is acquired by the current thread.
#[allow(unused)]
unsafe fn resume_if_panicking() {
    if let Some(err) = PANIC.take() {
        std::panic::resume_unwind(err);
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl<'a> EventLoop<'a> {
    /// Constructs a new `EventLoop`, using the provided `EventHandler`.\
    /// ## PANICS
    /// * Panics if another `EventLoop` object exists and holds the Event Thread lock.
    /// * Panics if a previous `EventLoop` panicked while holding the Event Thread lock.
    /// * MacOS: Panics if the Calling Thread is not the Main Thread.
    pub fn new(handler: &'a dyn EventHandler) -> Self {
        let lock = match EVENTLOOP_MUTEX.try_lock() {
            Ok(lock) => lock,
            Err(TryLockError::WouldBlock) => {
                panic!("Attempted to create multiple Event Loops simultaneously!")
            }
            Err(TryLockError::Poisoned(_)) => {
                panic!("Attempted to create a poisoned Event Loop after panic!")
            }
        };

        let connection = XcbConnection::new();
        let epoller = Epoller::new(connection.fd());
        let atoms = XcbAtomList::new(&connection).unwrap();

        Self {
            handler,
            atoms,
            epoller,
            connection,
            lock,
        }
    }
}

impl Drop for EventLoop<'_> {
    fn drop(&mut self) {}
}

// ================================================================================================================================ //

impl EventLoop<'_> {
    /// Starts a native Event/Message loop.
    /// ## PANICS
    /// * Panics if the Calling Thread is not the Event Thread.
    /// * Panics if User-Code panics in an `EventHandler` callback.
    pub fn run(&self) {
        // SAFETY: This function is called only on the Event Thread.
        let guard = EventLoopGuard::new(self);

        // SAFETY: This function is called while holding an `EventLoopGuard`.
        unsafe { guard.events.internal_run() };

        // SAFETY: This function is called while holding an `EventLoopGuard`.
        unsafe { resume_if_panicking() };
    }

    /// The internal native Event/Message Loop.
    /// ## SAFETY
    /// * Must be called on the Event Thread to function properly.
    /// * Must be called while holding an `EventLoopGuard` to ensure the validity of the static EVENTLOOP.
    unsafe fn internal_run(&self) {
        self.handler.start(self);

        while let Some(event) = self.connection.next_xcb_event(&self.epoller) {
            let ev_type = event.variant();
            let ev_flag = event.flag();
            let ev_char = if ev_flag { '*' } else { ' ' };

            let ev_name = log::xcb_generic_event_name(event.as_ref());
            eprintln!("-- [EVENT] [{ev_char}{ev_type:2}] \"{ev_name}\" : {event:?}");

            match event.enumerate() {
                XcbEventRef::ClientMessage(evt) => {
                    let p1 = (event.as_ref() as *const _) as usize;
                    let p2 = (evt as *const _ as *const _) as usize;
                    assert_eq!(p1, p2);

                    if evt.data.data32[0] == self.atoms.wm_delete_window.get() {
                        // Closes window, just in case the User callback `panic`s.
                        let closer = defer(|| {
                            let cookie = unsafe {
                                sys::xcb_destroy_window_checked(self.connection.handle, evt.window)
                            };
                            // Check for errors, only if not panicking.
                            if !std::thread::panicking() {
                                self.connection.check_cookie(cookie).unwrap();
                            }
                        });

                        self.handler.window_close(self, evt.window);

                        drop(closer);
                    }
                }
                XcbEventRef::ConfigureNotify(evt) => {
                    self.handler.window_reposition(self, evt.window);
                }
                XcbEventRef::FocusIn(evt) => {
                    self.handler.window_focus(self, evt.event, true);
                }
                XcbEventRef::FocusOut(evt) => {
                    self.handler.window_focus(self, evt.event, false);
                }
                XcbEventRef::MotionNotify(evt) => {
                    let xcb_pt = sys::xcb_point_t {
                        x: evt.event_x,
                        y: evt.event_y,
                    };
                    let pt = Point::from(xcb_pt);
                    self.handler.cursor_move(self, evt.event, pt);
                }
                XcbEventRef::ButtonPress(evt) => {
                    match evt.detail {
                        4 => self.handler.scroll_wheel(self, evt.event, 0.0, 1.0),
                        5 => self.handler.scroll_wheel(self, evt.event, 0.0, -1.0),
                        6 => self.handler.scroll_wheel(self, evt.event, -1.0, 0.0),
                        7 => self.handler.scroll_wheel(self, evt.event, 1.0, 0.0),
                        _ => {
                            let button = MouseButton(evt.detail as _);
                            self.handler.button_press(self, evt.event, button, true);
                        }
                    };
                }
                XcbEventRef::ButtonRelease(evt) => {
                    match evt.detail {
                        4..=7 => {}
                        _ => {
                            let button = MouseButton(evt.detail as _);
                            self.handler.button_press(self, evt.event, button, false);
                        }
                    };
                }
                XcbEventRef::KeyPress(evt) => {
                    let keycode = KeyCode(evt.detail as _);
                    self.handler.key_press(self, evt.event, keycode, true);
                }
                XcbEventRef::KeyRelease(evt) => {
                    let keycode = KeyCode(evt.detail as _);
                    self.handler.key_press(self, evt.event, keycode, false);
                }
                _ => {}
            }
        }
        self.connection.status().unwrap();

        self.handler.stop(self);
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

impl EventLoop<'_> {
    /// Returns a boolean indicating whether or not the Calling Thread is the Event Thread.
    pub fn is_this_thread(&self) -> bool {
        todo!()
    }

    /// Returns a boolean indicating whether or not the Event Thread is running an Event/Message Loop.
    pub fn is_running(&self) -> bool {
        self.connection.status().is_ok()
    }

    /// If an Event/Message Loop is running, then request it to terminate.
    pub fn request_stop(&self) {
        self.epoller.signal_quit();
    }
}

// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //
// ================================================================================================================================ //

/// Watches multiple File Descriptors and can wait until any of them have data to read.
pub(crate) struct Epoller {
    /// The File Descriptor that can call `epoll` functions.
    epoll_fd: OwnedFd,
    /// Signals the Event Loop should Quit.
    quit_fd: OwnedFd,
    /// Signals the Event Loop should Execute a user-callback.
    exec_fd: OwnedFd,
    /// Signals the Xcb Connection may have new events.
    conn_fd: RawFd,
}

/// The result of waiting on a set of File Descriptors.
pub(crate) enum EpollResult {
    /// No events were written (or too many), but the wait returned.
    Failure,
    /// Some event was written, but was unrecognized.
    Unknown,
    /// The Xcb Connection has data.
    Conn,
    /// The `execute` command was signaled.
    Exec,
    /// The `quit` command was signaled.
    Quit,
}

impl Epoller {
    /// Constructs a new Epoller and associated File Descriptors.
    pub fn new(conn_fd: RawFd) -> Self {
        let epoll_fd = unsafe { OwnedFd::from_raw_fd(sys::epoll_create1(0)) };
        let quit_fd = unsafe { OwnedFd::from_raw_fd(sys::eventfd(0, sys::EFD_NONBLOCK)) };
        let exec_fd = unsafe { OwnedFd::from_raw_fd(sys::eventfd(0, sys::EFD_NONBLOCK)) };

        let this = Self {
            epoll_fd,
            quit_fd,
            exec_fd,
            conn_fd,
        };

        this.register(this.conn_fd.as_raw_fd());
        this.register(this.exec_fd.as_raw_fd());
        this.register(this.quit_fd.as_raw_fd());

        this
    }

    /// Registers a File Descriptor to be available for watching.
    fn register(&self, fd: RawFd) {
        let mut event = sys::epoll_event {
            events: sys::EPOLLIN as u32,
            data: sys::epoll_data_t { fd },
        };

        let res = unsafe {
            sys::epoll_ctl(
                self.epoll_fd.as_raw_fd(),
                sys::EPOLL_CTL_ADD,
                fd,
                event.as_libc_mut(),
            )
        };
        assert_ne!(res, -1);
    }
}

impl Epoller {
    /// Writes `data` into an EventFd.
    fn write(&self, fd: RawFd, data: u64) {
        let res = unsafe { sys::write(fd, void_of!(data), 8) };
        assert_ne!(res, -1)
    }

    /// Reads `data` from an EventFd.
    fn read(&self, fd: RawFd) -> u64 {
        let mut data = 0u64;
        let res = unsafe { sys::read(fd, void_of_mut!(data), 8) };
        assert_eq!(res, 8);
        data
    }

    /// Signals the ExecFd.
    #[allow(unused)]
    pub fn signal_exec(&self) {
        self.write(self.exec_fd.as_raw_fd(), 1)
    }

    /// Resets the ExecFd.
    fn reset_exec(&self) {
        let _ = self.read(self.exec_fd.as_raw_fd());
    }

    /// Signals the QuitFd.
    pub fn signal_quit(&self) {
        self.write(self.quit_fd.as_raw_fd(), 1)
    }

    /// Signals the QuitFd.
    fn reset_quit(&self) {
        let _ = self.read(self.quit_fd.as_raw_fd());
    }

    /// Waits for one of the File Descriptors to be ready, and returns which one has data.
    fn wait(&self) -> EpollResult {
        let mut event: sys::epoll_event = unsafe { zeroed() };

        let count =
            unsafe { sys::epoll_wait(self.epoll_fd.as_raw_fd(), event.as_libc_mut(), 1, -1) };
        assert_ne!(count, -1);

        if count == 1 {
            let fd = unsafe { event.data.fd };

            if fd == self.conn_fd.as_raw_fd() {
                EpollResult::Conn
            } else if fd == self.exec_fd.as_raw_fd() {
                self.reset_exec();
                EpollResult::Exec
            } else if fd == self.quit_fd.as_raw_fd() {
                self.reset_quit();
                EpollResult::Quit
            } else {
                EpollResult::Unknown
            }
        } else {
            EpollResult::Failure
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Wrapper for XCB Connections.
#[allow(unused)]
pub(crate) struct XcbConnection {
    /// The underlying connection.
    pub(crate) handle: *mut sys::xcb_connection_t,

    /// The ID of the screen associated with this connection.
    pub(crate) screen_id: i32,

    /// Data associated with this connection.
    pub(crate) data: *const sys::xcb_setup_t,
}

// ---------------------------------------------------------------- //

impl XcbConnection {
    /// Opens a new XCB Connection.
    fn new() -> Self {
        let mut screen_id = 0;
        let handle = unsafe { sys::xcb_connect(null(), addr_of_mut!(screen_id)) };
        assert!(Self::connection_status(handle).is_ok());

        let data = unsafe { sys::xcb_get_setup(handle) };
        assert!(!data.is_null());

        Self {
            handle,
            screen_id,
            data,
        }
    }
}

impl Drop for XcbConnection {
    fn drop(&mut self) {
        unsafe { sys::xcb_disconnect(self.handle) };
    }
}

// ---------------------------------------------------------------- //

impl XcbConnection {
    /// Retrieves the Status of the connection.
    fn connection_status(connection: *mut sys::xcb_connection_t) -> XcbConnectionResult<()> {
        let res = unsafe { sys::xcb_connection_has_error(connection) };
        match XcbConnectionError::new(res) {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

    /// Retrieves the File Descriptor for the connection.
    pub(crate) fn fd(&self) -> RawFd {
        let fd = unsafe { sys::xcb_get_file_descriptor(self.handle) };
        assert_ne!(fd, -1);
        fd
    }

    /// Retrieves the Status of this connection.
    pub(crate) fn status(&self) -> XcbConnectionResult<()> {
        Self::connection_status(self.handle)
    }

    /// Retrieves Screen Data.
    pub(crate) fn screen(&self) -> &sys::xcb_screen_t {
        let mut iter = unsafe { sys::xcb_setup_roots_iterator(self.data) };
        for _ in 0..self.screen_id {
            assert_ne!(iter.rem, 0);
            unsafe { sys::xcb_screen_next(addr_of_mut!(iter)) };
        }
        unsafe { iter.data.as_ref() }.expect("Screen should not be NULL.")
    }

    /// Checks a cookie result and returns an appropriate error, if any.
    pub(crate) fn check_cookie(&self, cookie: sys::xcb_void_cookie_t) -> XcbGenericResult<()> {
        let res = unsafe { sys::xcb_request_check(self.handle, cookie) };
        match XcbGenericError::new(res) {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

    /// Flushes commands sent over the connection.
    pub(crate) fn flush(&self) {
        let res = unsafe { sys::xcb_flush(self.handle) };
        assert_ne!(res, 0);
    }

    // ---------------------------------------------------------------- //

    /// Retrieves an atom for the given string.
    pub fn query_atom(&self, name: &str, register: bool) -> XcbGenericResult<OptionalXcbAtom> {
        let name_len = name.len() as u16;
        let name_dat = name.as_ptr() as *const c_char;

        let mut error = null_mut();
        let cookie =
            unsafe { sys::xcb_intern_atom(self.handle, (!register) as u8, name_len, name_dat) };
        let reply = unsafe { sys::xcb_intern_atom_reply(self.handle, cookie, addr_of_mut!(error)) };
        let _reply_free = defer(|| {
            if !reply.is_null() {
                unsafe { sys::free(reply as *mut c_void) };
            }
        });

        match XcbGenericError::new(error) {
            Some(err) => Err(err),
            None => match unsafe { reply.as_ref() } {
                Some(rep) => match NonzeroXcbAtom::new(rep.atom) {
                    Some(atom) => Ok(Some(atom)),
                    None => Ok(None),
                },
                None => Ok(None),
            },
        }
    }

    /// Retrieves the next Xcb Event.
    fn next_xcb_event(&self, epoller: &Epoller) -> Option<XcbEvent> {
        //return XcbEvent::new(unsafe { sys::xcb_wait_for_event(self.handle) });

        if let Some(event) = XcbEvent::new(unsafe { sys::xcb_poll_for_event(self.handle) }) {
            return Some(event);
        }

        loop {
            let res = epoller.wait();
            match res {
                EpollResult::Failure => unimplemented!(),
                EpollResult::Unknown => unimplemented!(),
                EpollResult::Quit => {
                    eprintln!("[EPOLL QUIT]");
                    return None;
                }
                EpollResult::Exec => {
                    eprintln!("[EPOLL EXEC]");
                }
                EpollResult::Conn => {
                    let event = unsafe { sys::xcb_poll_for_event(self.handle) };
                    if let Some(event) = XcbEvent::new(event) {
                        return Some(event);
                    } else {
                        let status = self.status();
                        match status {
                            Ok(_) => {
                                eprintln!("[EPOLL NO EVENT]");
                            }
                            Err(_) => {
                                eprintln!("[EPOLL ERROR]");
                                status.unwrap();
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// A Non-Zero Xcb Atom.
pub(crate) type NonzeroXcbAtom = NonZero<sys::xcb_atom_t>;

/// An Optional Non-Zero Xcb Atom.
pub(crate) type OptionalXcbAtom = Option<NonzeroXcbAtom>;

/// A list of Xcb Atoms.
pub(crate) struct XcbAtomList {
    /// Custom XID for identifying Wyn.
    pub _wyn_xid: NonzeroXcbAtom,

    /// Extended Window Manager Hints.
    _ewmh: UnsafeCell<sys::xcb_ewmh_connection_t>,

    /// WM_DELETE_WINDOW Atom.
    pub wm_delete_window: NonzeroXcbAtom,
}

impl XcbAtomList {
    /// Reference to EWMH data.
    pub fn ewmh_ref(&self) -> &sys::xcb_ewmh_connection_t {
        unsafe { &*(self._ewmh.get() as *const sys::xcb_ewmh_connection_t) }
    }

    /// Pointer to EWMH data.
    pub fn ewmh_ptr(&self) -> *mut sys::xcb_ewmh_connection_t {
        self._ewmh.get()
    }
}

impl XcbAtomList {
    /// Queries the available atoms and stores them.
    pub fn new(connection: &XcbConnection) -> XcbGenericResult<Self> {
        let _wyn_xid = {
            let handle = unsafe { sys::xcb_generate_id(connection.handle) };
            NonzeroXcbAtom::new(handle).expect("Unable to generate <WYN_XID>!")
        };

        let mut error = null_mut();
        let mut ewmh = unsafe { zeroed() };
        let cookie = unsafe { sys::xcb_ewmh_init_atoms(connection.handle, addr_of_mut!(ewmh)) };
        let res = unsafe {
            sys::xcb_ewmh_init_atoms_replies(addr_of_mut!(ewmh), cookie, addr_of_mut!(error))
        };
        assert!(XcbGenericError::new(error).is_none());
        assert_eq!(res, 1);

        let _ewmh = UnsafeCell::new(ewmh);

        let wm_delete_window = connection
            .query_atom("WM_DELETE_WINDOW", false)
            .expect("XCB ATOM QUERY FAILURE")
            .expect("XCB ATOM NULL FAILURE");

        Ok(Self {
            _wyn_xid,
            _ewmh,
            wm_delete_window,
        })
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Native-type for XCB Events.
type NativeXcbEvent = sys::xcb_generic_event_t;

/// Handle-type for XCB Events.
type NativeXcbEventHandle = *mut NativeXcbEvent;

/// Nonzero-type for XCB Events.
type NonzeroXcbEventHandle = NonNull<NativeXcbEvent>;

/// Wrapper for XCB Events.
#[repr(transparent)]
struct XcbEvent(NonzeroXcbEventHandle);

/// A reference to the underlying XCB Event.
#[repr(u8)]
#[allow(clippy::missing_docs_in_private_items)]
enum XcbEventRef<'a> {
    Generic(&'a sys::xcb_generic_event_t) = 0u8,
    KeyPress(&'a sys::xcb_key_press_event_t) = sys::XCB_KEY_PRESS as u8,
    KeyRelease(&'a sys::xcb_key_release_event_t) = sys::XCB_KEY_RELEASE as u8,
    ButtonPress(&'a sys::xcb_button_press_event_t) = sys::XCB_BUTTON_PRESS as u8,
    ButtonRelease(&'a sys::xcb_button_release_event_t) = sys::XCB_BUTTON_RELEASE as u8,
    MotionNotify(&'a sys::xcb_motion_notify_event_t) = sys::XCB_MOTION_NOTIFY as u8,
    EnterNotify(&'a sys::xcb_enter_notify_event_t) = sys::XCB_ENTER_NOTIFY as u8,
    LeaveNotify(&'a sys::xcb_leave_notify_event_t) = sys::XCB_LEAVE_NOTIFY as u8,
    FocusIn(&'a sys::xcb_focus_in_event_t) = sys::XCB_FOCUS_IN as u8,
    FocusOut(&'a sys::xcb_focus_out_event_t) = sys::XCB_FOCUS_OUT as u8,
    KeymapNotify(&'a sys::xcb_keymap_notify_event_t) = sys::XCB_KEYMAP_NOTIFY as u8,
    Expose(&'a sys::xcb_expose_event_t) = sys::XCB_EXPOSE as u8,
    GraphicsExposure(&'a sys::xcb_graphics_exposure_event_t) = sys::XCB_GRAPHICS_EXPOSURE as u8,
    NoExposure(&'a sys::xcb_no_exposure_event_t) = sys::XCB_NO_EXPOSURE as u8,
    VisibilityNotify(&'a sys::xcb_visibility_notify_event_t) = sys::XCB_VISIBILITY_NOTIFY as u8,
    CreateNotify(&'a sys::xcb_create_notify_event_t) = sys::XCB_CREATE_NOTIFY as u8,
    DestroyNotify(&'a sys::xcb_destroy_notify_event_t) = sys::XCB_DESTROY_NOTIFY as u8,
    UnmapNotify(&'a sys::xcb_unmap_notify_event_t) = sys::XCB_UNMAP_NOTIFY as u8,
    MapNotify(&'a sys::xcb_map_notify_event_t) = sys::XCB_MAP_NOTIFY as u8,
    MapRequest(&'a sys::xcb_map_request_event_t) = sys::XCB_MAP_REQUEST as u8,
    ReparentNotify(&'a sys::xcb_reparent_notify_event_t) = sys::XCB_REPARENT_NOTIFY as u8,
    ConfigureNotify(&'a sys::xcb_configure_notify_event_t) = sys::XCB_CONFIGURE_NOTIFY as u8,
    ConfigureRequest(&'a sys::xcb_configure_request_event_t) = sys::XCB_CONFIGURE_REQUEST as u8,
    GravityNotify(&'a sys::xcb_gravity_notify_event_t) = sys::XCB_GRAVITY_NOTIFY as u8,
    ResizeRequest(&'a sys::xcb_resize_request_event_t) = sys::XCB_RESIZE_REQUEST as u8,
    CirculateNotify(&'a sys::xcb_circulate_notify_event_t) = sys::XCB_CIRCULATE_NOTIFY as u8,
    CirculateRequest(&'a sys::xcb_circulate_request_event_t) = sys::XCB_CIRCULATE_REQUEST as u8,
    PropertyNotify(&'a sys::xcb_property_notify_event_t) = sys::XCB_PROPERTY_NOTIFY as u8,
    SelectionClear(&'a sys::xcb_selection_clear_event_t) = sys::XCB_SELECTION_CLEAR as u8,
    SelectionRequest(&'a sys::xcb_selection_request_event_t) = sys::XCB_SELECTION_REQUEST as u8,
    SelectionNotify(&'a sys::xcb_selection_notify_event_t) = sys::XCB_SELECTION_NOTIFY as u8,
    ColormapNotify(&'a sys::xcb_colormap_notify_event_t) = sys::XCB_COLORMAP_NOTIFY as u8,
    ClientMessage(&'a sys::xcb_client_message_event_t) = sys::XCB_CLIENT_MESSAGE as u8,
    MappingNotify(&'a sys::xcb_mapping_notify_event_t) = sys::XCB_MAPPING_NOTIFY as u8,
    GeGeneric(&'a sys::xcb_ge_generic_event_t) = sys::XCB_GE_GENERIC as u8,
}

impl Debug for XcbEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.enumerate() {
            XcbEventRef::Generic(ev) => write!(f, "{ev:?}"),
            XcbEventRef::KeyPress(ev) => write!(f, "{ev:?}"),
            XcbEventRef::KeyRelease(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ButtonPress(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ButtonRelease(ev) => write!(f, "{ev:?}"),
            XcbEventRef::MotionNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::EnterNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::LeaveNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::FocusIn(ev) => write!(f, "{ev:?}"),
            XcbEventRef::FocusOut(ev) => write!(f, "{ev:?}"),
            XcbEventRef::KeymapNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::Expose(ev) => write!(f, "{ev:?}"),
            XcbEventRef::GraphicsExposure(ev) => write!(f, "{ev:?}"),
            XcbEventRef::NoExposure(ev) => write!(f, "{ev:?}"),
            XcbEventRef::VisibilityNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::CreateNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::DestroyNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::UnmapNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::MapNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::MapRequest(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ReparentNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ConfigureNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ConfigureRequest(ev) => write!(f, "{ev:?}"),
            XcbEventRef::GravityNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ResizeRequest(ev) => write!(f, "{ev:?}"),
            XcbEventRef::CirculateNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::CirculateRequest(ev) => write!(f, "{ev:?}"),
            XcbEventRef::PropertyNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::SelectionClear(ev) => write!(f, "{ev:?}"),
            XcbEventRef::SelectionRequest(ev) => write!(f, "{ev:?}"),
            XcbEventRef::SelectionNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ColormapNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::MappingNotify(ev) => write!(f, "{ev:?}"),
            XcbEventRef::GeGeneric(ev) => write!(f, "{ev:?}"),
            XcbEventRef::ClientMessage(ev) => {
                let mut db = f.debug_struct("xcb_client_message_event_t");
                db.field("response_type", &ev.response_type);
                db.field("format", &ev.format);
                db.field("sequence", &ev.sequence);
                db.field("window", &ev.window);
                db.field("type_", &ev.type_);
                unsafe {
                    match ev.format {
                        8 => db.field("data8", &ev.data.data8),
                        16 => db.field("data16", &ev.data.data16),
                        32 => db.field("data32", &ev.data.data32),
                        _ => return db.finish_non_exhaustive(),
                    }
                };
                db.finish()
            }
        }
    }
}

impl XcbEvent {
    /// Constructs a new `XcbEvent`.
    fn new(event: NativeXcbEventHandle) -> Option<Self> {
        NonzeroXcbEventHandle::new(event).map(Self)
    }

    /// Returns a reference to the underlying event.
    fn as_ref(&self) -> &NativeXcbEvent {
        unsafe { self.0.as_ref() }
    }

    /// The type of event.
    fn variant(&self) -> u8 {
        self.as_ref().response_type & 0x7F
    }

    /// Whether or not the flag is set.
    fn flag(&self) -> bool {
        self.as_ref().response_type & 0x80 != 0
    }

    /// Gets the actual underlying event, if known.
    fn enumerate(&self) -> XcbEventRef {
        let ptr = self.0.as_ptr();

        let event = unsafe {
            match self.variant() as u32 {
                sys::XCB_KEY_PRESS => XcbEventRef::KeyPress(&*(ptr as *const _)),
                sys::XCB_KEY_RELEASE => XcbEventRef::KeyRelease(&*(ptr as *const _)),
                sys::XCB_BUTTON_PRESS => XcbEventRef::ButtonPress(&*(ptr as *const _)),
                sys::XCB_BUTTON_RELEASE => XcbEventRef::ButtonRelease(&*(ptr as *const _)),
                sys::XCB_MOTION_NOTIFY => XcbEventRef::MotionNotify(&*(ptr as *const _)),
                sys::XCB_ENTER_NOTIFY => XcbEventRef::EnterNotify(&*(ptr as *const _)),
                sys::XCB_LEAVE_NOTIFY => XcbEventRef::LeaveNotify(&*(ptr as *const _)),
                sys::XCB_FOCUS_IN => XcbEventRef::FocusIn(&*(ptr as *const _)),
                sys::XCB_FOCUS_OUT => XcbEventRef::FocusOut(&*(ptr as *const _)),
                sys::XCB_KEYMAP_NOTIFY => XcbEventRef::KeymapNotify(&*(ptr as *const _)),
                sys::XCB_EXPOSE => XcbEventRef::Expose(&*(ptr as *const _)),
                sys::XCB_GRAPHICS_EXPOSURE => XcbEventRef::GraphicsExposure(&*(ptr as *const _)),
                sys::XCB_NO_EXPOSURE => XcbEventRef::NoExposure(&*(ptr as *const _)),
                sys::XCB_VISIBILITY_NOTIFY => XcbEventRef::VisibilityNotify(&*(ptr as *const _)),
                sys::XCB_CREATE_NOTIFY => XcbEventRef::CreateNotify(&*(ptr as *const _)),
                sys::XCB_DESTROY_NOTIFY => XcbEventRef::DestroyNotify(&*(ptr as *const _)),
                sys::XCB_UNMAP_NOTIFY => XcbEventRef::UnmapNotify(&*(ptr as *const _)),
                sys::XCB_MAP_NOTIFY => XcbEventRef::MapNotify(&*(ptr as *const _)),
                sys::XCB_MAP_REQUEST => XcbEventRef::MapRequest(&*(ptr as *const _)),
                sys::XCB_REPARENT_NOTIFY => XcbEventRef::ReparentNotify(&*(ptr as *const _)),
                sys::XCB_CONFIGURE_NOTIFY => XcbEventRef::ConfigureNotify(&*(ptr as *const _)),
                sys::XCB_CONFIGURE_REQUEST => XcbEventRef::ConfigureRequest(&*(ptr as *const _)),
                sys::XCB_GRAVITY_NOTIFY => XcbEventRef::GravityNotify(&*(ptr as *const _)),
                sys::XCB_RESIZE_REQUEST => XcbEventRef::ResizeRequest(&*(ptr as *const _)),
                sys::XCB_CIRCULATE_NOTIFY => XcbEventRef::CirculateNotify(&*(ptr as *const _)),
                sys::XCB_CIRCULATE_REQUEST => XcbEventRef::CirculateRequest(&*(ptr as *const _)),
                sys::XCB_PROPERTY_NOTIFY => XcbEventRef::PropertyNotify(&*(ptr as *const _)),
                sys::XCB_SELECTION_CLEAR => XcbEventRef::SelectionClear(&*(ptr as *const _)),
                sys::XCB_SELECTION_REQUEST => XcbEventRef::SelectionRequest(&*(ptr as *const _)),
                sys::XCB_SELECTION_NOTIFY => XcbEventRef::SelectionNotify(&*(ptr as *const _)),
                sys::XCB_COLORMAP_NOTIFY => XcbEventRef::ColormapNotify(&*(ptr as *const _)),
                sys::XCB_CLIENT_MESSAGE => XcbEventRef::ClientMessage(&*(ptr as *const _)),
                sys::XCB_MAPPING_NOTIFY => XcbEventRef::MappingNotify(&*(ptr as *const _)),
                sys::XCB_GE_GENERIC => XcbEventRef::GeGeneric(&*(ptr as *const _)),
                _ => XcbEventRef::Generic(&*(ptr as *const _)),
            }
        };

        event
    }
}

impl Drop for XcbEvent {
    fn drop(&mut self) {
        // SAFETY: Event pointer was allocated, must be freed: <https://xcb.freedesktop.org/tutorial/events/#receivingevents:writingtheeventsloop>
        unsafe { sys::free(self.0.as_ptr() as *mut c_void) };
    }
}

// ================================================================================================================================ //
