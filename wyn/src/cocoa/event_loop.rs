/*
 *  Crate: Wyn
 * Module: Cocoa - Event Loop
 */

//! ...

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use super::events::EventHandler;
use super::inputs::*;
use super::types::*;

use std::sync::{Mutex, MutexGuard, TryLockError};

// ================================================================================================================================ //

/// Holds the state/context required to run a native Event/Message Loop.\
/// Only one `EventLoop` may exist at any given moment, and it may or may not be running.
/// ## Notes
/// On some platforms, it is required to run the `EventLoop` on the Main Thread.
pub struct EventLoop<'a> {
    /// The Event Handler that responds to events.
    pub(crate) handler: &'a dyn EventHandler,

    #[allow(unused)]
    /// The Obj-C application delegate.
    pub(crate) delegate: *mut sys::NSApplicationDelegate,

    /// The Obj-C shared application.
    pub(crate) app: *mut sys::NSApplication,

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
        if sys::NO == unsafe { sys::ns_thread::isMainThread_class() } {
            panic!("Attempted to create an Event Loop while not on the Main Thread!")
        }

        let lock = match EVENTLOOP_MUTEX.try_lock() {
            Ok(lock) => lock,
            Err(TryLockError::WouldBlock) => {
                panic!("Attempted to create multiple Event Loops simultaneously!")
            }
            Err(TryLockError::Poisoned(_)) => {
                panic!("Attempted to create a poisoned Event Loop after panic!")
            }
        };

        let app = unsafe { sys::ns_application::sharedApplication() };
        let _res = unsafe {
            sys::ns_application::setActivationPolicy_(
                app,
                sys::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
            )
        };

        let delegate = wyn_delegate::new();
        unsafe { sys::ns_application::setDelegate_(app, delegate) };

        Self {
            handler,
            delegate,
            app,
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
        unsafe { guard.events.internal_run() }
    }

    /// The internal native Event/Message Loop.
    /// ## SAFETY
    /// * Must be called on the Event Thread to function properly.
    /// * Must be called while holding an `EventLoopGuard` to ensure the validity of the static EVENTLOOP.
    unsafe fn internal_run(&self) {
        sys::ns_application::run(self.app);

        sys::ns_event::stopPeriodicEvents();
        resume_if_panicking();

        self.handler.stop(self);
    }
}

// ================================================================================================================================ //

impl EventLoop<'_> {
    /// Returns a boolean indicating whether or not the Calling Thread is the Event Thread.
    pub fn is_this_thread(&self) -> bool {
        sys::NO != unsafe { sys::ns_thread::isMainThread_class() }
    }

    /// Returns a boolean indicating whether or not the Event Thread is running an Event/Message Loop.
    pub fn is_running(&self) -> bool {
        sys::NO != unsafe { sys::ns_application::running(self.app) }
    }

    /// If an Event/Message Loop is running, then request it to terminate.
    pub fn request_stop(&self) {
        unsafe {
            sys::ns_application::stop_(self.app, self.app);
            sys::ns_event::stopPeriodicEvents();
            sys::ns_event::startPeriodicEventsAfterDelay_withPeriod_(0.0, 0.0);
        };
    }
}

// ================================================================================================================================ //

#[allow(non_snake_case)]
/// `WynDelegate : NSObject <NSApplicationDelegate, NSWindowDelegate, NSResponder>`
pub mod wyn_delegate {
    use super::*;
    use sys::*;

    use std::sync::Once;

    // ---------------------------------------------------------------- //

    /// Registers the Class with the Objective-C Runtime (if it hasn't been done previously) and then returns a reference to the Class object.
    fn class() -> &'static Class {
        /// Synchronization Object to ensure the Class is registed only Once.
        static ONCE: Once = Once::new();
        /// The Class Object representing a `WynDelegate` instance.
        static mut CLASS: MaybeUninit<&'static Class> = MaybeUninit::uninit();

        let register_class = || {
            let mut decl = ClassDecl::new("WynDelegate", class!(NSView))
                .expect("Unable to register `WynDelegate` class.");

            // NSApplicationDelegate
            unsafe {
                decl.add_method(
                    sel!(applicationDidFinishLaunching:),
                    applicationDidFinishLaunching_
                        as extern "C" fn(&NSApplicationDelegate, Sel, *mut NSNotification),
                );
                decl.add_method(
                    sel!(applicationShouldTerminate:),
                    applicationShouldTerminate_
                        as extern "C" fn(
                            &NSApplicationDelegate,
                            Sel,
                            *mut NSApplication,
                        ) -> NSUInteger,
                );
            };

            // NSWindowDelegate
            unsafe {
                decl.add_method(
                    sel!(windowWillClose:),
                    windowWillClose_ as extern "C" fn(&NSWindowDelegate, Sel, *mut NSNotification),
                );
                decl.add_method(
                    sel!(windowDidMove:),
                    windowDidMove_ as extern "C" fn(&NSWindowDelegate, Sel, *mut NSNotification),
                );
                decl.add_method(
                    sel!(windowDidResize:),
                    windowDidResize_ as extern "C" fn(&NSWindowDelegate, Sel, *mut NSNotification),
                );
                decl.add_method(
                    sel!(windowDidBecomeKey:),
                    windowDidBecomeKey_
                        as extern "C" fn(&NSWindowDelegate, Sel, *mut NSNotification),
                );
                decl.add_method(
                    sel!(windowDidResignKey:),
                    windowDidResignKey_
                        as extern "C" fn(&NSWindowDelegate, Sel, *mut NSNotification),
                );
            };

            // NSView
            unsafe {
                decl.add_method(
                    sel!(acceptsFirstMouse:),
                    acceptsFirstMouse_ as extern "C" fn(&NSView, Sel, *mut NSEvent) -> BOOL,
                );
            };

            // NSResponder
            unsafe {
                decl.add_method(
                    sel!(mouseDown:),
                    mouseDown_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(rightMouseDown:),
                    rightMouseDown_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(otherMouseDown:),
                    otherMouseDown_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(mouseUp:),
                    mouseUp_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(rightMouseUp:),
                    rightMouseUp_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(otherMouseUp:),
                    otherMouseUp_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(mouseDragged:),
                    mouseDragged_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(rightMouseDragged:),
                    rightMouseDragged_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(otherMouseDragged:),
                    otherMouseDragged_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(mouseMoved:),
                    mouseMoved_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );

                decl.add_method(
                    sel!(scrollWheel:),
                    scrollWheel_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );

                decl.add_method(
                    sel!(mouseEntered:),
                    mouseEntered_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(mouseExited:),
                    mouseExited_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );

                decl.add_method(
                    sel!(keyDown:),
                    keyDown_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
                decl.add_method(
                    sel!(keyUp:),
                    keyUp_ as extern "C" fn(&NSResponder, Sel, *mut NSEvent),
                );
            };

            let class = decl.register();
            unsafe { CLASS.write(class) };
        };

        ONCE.call_once(register_class);
        assert!(ONCE.is_completed());

        unsafe { CLASS.assume_init() }
    }

    // ---------------------------------------------------------------- //

    /// `- (void)applicationDidFinishLaunching:(NSNotification *)notification`
    /// <https://developer.apple.com/documentation/appkit/nsapplicationdelegate/1428385-applicationdidfinishlaunching?language=objc>
    extern "C" fn applicationDidFinishLaunching_(
        _this: &NSApplicationDelegate,
        _cmd: Sel,
        _notification: *mut NSNotification,
    ) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                unsafe { sys::ns_application::activateIgnoringOtherApps_(events.app, YES) };
                events.handler.start(events);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (NSApplicationTerminateReply)applicationShouldTerminate:(NSApplication*)sender`
    /// https://developer.apple.com/documentation/appkit/nsapplicationdelegate/1428642-applicationshouldterminate?language=objc
    extern "C" fn applicationShouldTerminate_(
        _this: &NSApplicationDelegate,
        _cmd: Sel,
        _sender: *mut NSApplication,
    ) -> NSUInteger {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                eprintln!("[APP TERMINATING]");
                events.request_stop();
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }

        NSApplicationTerminateReply::NSTerminateCancel as NSUInteger
        //NSApplicationTerminateReply::NSTerminateNow as NSUInteger
    }

    // ---------------------------------------------------------------- //

    /// `- (void)windowWillClose:(NSNotification *)notification`
    /// <https://developer.apple.com/documentation/appkit/nswindowdelegate/1419605-windowwillclose?language=objc>
    extern "C" fn windowWillClose_(
        _this: &NSWindowDelegate,
        _cmd: Sel,
        _notification: *mut NSNotification,
    ) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let handle = unsafe { sys::ns_notification::object(_notification) };
                events.handler.window_close(events, handle);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)windowDidMove:(NSNotification *)notification`
    /// <https://developer.apple.com/documentation/appkit/nswindowdelegate/1419674-windowdidmove?language=objc>
    extern "C" fn windowDidMove_(
        _this: &NSWindowDelegate,
        _cmd: Sel,
        _notification: *mut NSNotification,
    ) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let handle = unsafe { sys::ns_notification::object(_notification) };
                events.handler.window_reposition(events, handle);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)windowDidResize:(NSNotification *)notification`
    /// <https://developer.apple.com/documentation/appkit/nswindowdelegate/1419567-windowdidresize?language=objc>
    extern "C" fn windowDidResize_(
        _this: &NSWindowDelegate,
        _cmd: Sel,
        _notification: *mut NSNotification,
    ) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let handle = unsafe { sys::ns_notification::object(_notification) };
                events.handler.window_reposition(events, handle);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)windowDidBecomeKey:(NSNotification *)notification`
    /// <https://developer.apple.com/documentation/appkit/nswindowdelegate/1419737-windowdidbecomekey?language=objc>
    extern "C" fn windowDidBecomeKey_(
        _this: &NSWindowDelegate,
        _cmd: Sel,
        _notification: *mut NSNotification,
    ) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let handle = unsafe { sys::ns_notification::object(_notification) };

                unsafe { sys::ns_window::setContentView_(handle, events.delegate) };
                let res = unsafe { sys::ns_window::makeFirstResponder_(handle, events.delegate) };
                assert_eq!(res, sys::YES);

                events.handler.window_focus(events, handle, true);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)windowDidResignKey:(NSNotification *)notification`
    /// <https://developer.apple.com/documentation/appkit/nswindowdelegate/1419711-windowdidresignkey?language=objc>
    extern "C" fn windowDidResignKey_(
        _this: &NSWindowDelegate,
        _cmd: Sel,
        _notification: *mut NSNotification,
    ) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let handle = unsafe { sys::ns_notification::object(_notification) };
                events.handler.window_focus(events, handle, false);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    // ---------------------------------------------------------------- //

    /// `- (BOOL)acceptsFirstMouse:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsview/1483410-acceptsfirstmouse?language=objc>
    extern "C" fn acceptsFirstMouse_(_this: &NSView, _cmd: Sel, _event: *mut NSEvent) -> BOOL {
        eprintln!("ACCEPT MOUSE");
        sys::YES
    }

    // ---------------------------------------------------------------- //

    /// `- (void)mouseDown:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1524634-mousedown?language=objc>
    extern "C" fn mouseDown_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_button = unsafe { sys::ns_event::buttonNumber(event) };
                let button = MouseButton(ns_button);
                events.handler.button_press(events, ns_window, button, true);
            }
        };
        eprintln!("MOUSE DOWN!");

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)rightMouseDown:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1524727-rightmousedown?language=objc>
    extern "C" fn rightMouseDown_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_button = unsafe { sys::ns_event::buttonNumber(event) };
                let button = MouseButton(ns_button);
                events.handler.button_press(events, ns_window, button, true);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)otherMouseDown:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1525719-othermousedown?language=objc>
    extern "C" fn otherMouseDown_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_button = unsafe { sys::ns_event::buttonNumber(event) };
                let button = MouseButton(ns_button);
                events.handler.button_press(events, ns_window, button, true);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)mouseUp:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1535349-mouseup?language=objc>
    extern "C" fn mouseUp_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_button = unsafe { sys::ns_event::buttonNumber(event) };
                let button = MouseButton(ns_button);
                events
                    .handler
                    .button_press(events, ns_window, button, false);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)rightMouseUp:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1526309-rightmouseup?language=objc>
    extern "C" fn rightMouseUp_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_button = unsafe { sys::ns_event::buttonNumber(event) };
                let button = MouseButton(ns_button);
                events
                    .handler
                    .button_press(events, ns_window, button, false);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)otherMouseUp:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1531343-othermouseup?language=objc>
    extern "C" fn otherMouseUp_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_button = unsafe { sys::ns_event::buttonNumber(event) };
                let button = MouseButton(ns_button);
                events
                    .handler
                    .button_press(events, ns_window, button, false);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)mouseMoved:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1525114-mousemoved?language=objc>
    extern "C" fn mouseMoved_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_pt = unsafe { sys::ns_event::locationInWindow(event) };
                let pt = Point::from(ns_pt);
                events.handler.cursor_move(events, ns_window, pt);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)mouseDragged:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1527420-mousedragged?language=objc>
    extern "C" fn mouseDragged_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_pt = unsafe { sys::ns_event::locationInWindow(event) };
                let pt = Point::from(ns_pt);
                events.handler.cursor_move(events, ns_window, pt);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)rightMouseDragged:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1529135-rightmousedragged?language=objc>
    extern "C" fn rightMouseDragged_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_pt = unsafe { sys::ns_event::locationInWindow(event) };
                let pt = Point::from(ns_pt);
                events.handler.cursor_move(events, ns_window, pt);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)otherMouseDragged:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1529804-othermousedragged?language=objc>
    extern "C" fn otherMouseDragged_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_pt = unsafe { sys::ns_event::locationInWindow(event) };
                let pt = Point::from(ns_pt);
                events.handler.cursor_move(events, ns_window, pt);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)mouseEntered:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1529306-mouseentered?language=objc>
    extern "C" fn mouseEntered_(_this: &NSResponder, _cmd: Sel, _event: *mut NSEvent) {}

    /// `- (void)mouseExited:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1527561-mouseexited?language=objc>
    extern "C" fn mouseExited_(_this: &NSResponder, _cmd: Sel, _event: *mut NSEvent) {}

    /// `- (void)scrollWheel:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1534192-scrollwheel?language=objc>
    extern "C" fn scrollWheel_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let dx = unsafe { sys::ns_event::scrollingDeltaX(event) };
                let dy = unsafe { sys::ns_event::scrollingDeltaY(event) };
                events.handler.scroll_wheel(events, ns_window, dx, dy);
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)keyDown:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1525805-keydown?language=objc>
    extern "C" fn keyDown_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_key = unsafe { sys::ns_event::keyCode(event) };
                let keycode = KeyCode(ns_key);
                events.handler.key_press(events, ns_window, keycode, true)
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    /// `- (void)keyUp:(NSEvent *)event`
    /// <https://developer.apple.com/documentation/appkit/nsresponder/1527436-keyup?language=objc>
    extern "C" fn keyUp_(_this: &NSResponder, _cmd: Sel, event: *mut NSEvent) {
        let func = || {
            if let Some(events) = unsafe { EVENTLOOP } {
                let ns_window = unsafe { sys::ns_event::window(event) };
                let ns_key = unsafe { sys::ns_event::keyCode(event) };
                let keycode = KeyCode(ns_key);
                events.handler.key_press(events, ns_window, keycode, false)
            }
        };

        if let Err(err) = std::panic::catch_unwind(func) {
            unsafe { store_panic(err) }
        }
    }

    // ---------------------------------------------------------------- //

    /// Constructs a new `WynDelegate`.
    pub fn new() -> id {
        unsafe { msg_send![class(), new] }
    }
}

// ================================================================================================================================ //
