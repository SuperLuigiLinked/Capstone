/*
 *  Crate: GUI-Sys
 * Module: MacOS - Cocoa
 */

//! MacOS Cocoa bindings.
//!
//! # Dependencies
//! * <https://docs.rs/objc/latest/objc/>
//! * <https://docs.rs/cocoa/latest/cocoa/>
//!
//! # Documentation
//! * <https://developer.apple.com/documentation/technologies>
//!     * <https://developer.apple.com/documentation/foundation?language=objc>
//!     * <https://developer.apple.com/documentation/appkit?language=objc>

// -------------------------------------------------------------------------------------------------------------------------------- //

use crate::common::c_types::*;

use super::objc::*;

// ================================================================================================================================ //
// Macros
// -------------------------------------------------------------------------------------------------------------------------------- //

// ================================================================================================================================ //
// Types
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/objectivec/nsobject?language=objc>
id_class!(NSObject);

/// <https://developer.apple.com/documentation/foundation/nsstring?language=objc>
id_class!(NSString);

/// <https://developer.apple.com/documentation/foundation/nsthread?language=objc>
id_class!(NSThread);

/// <https://developer.apple.com/documentation/foundation/nsnotification?language=objc>
id_class!(NSNotification);

/// <https://developer.apple.com/documentation/appkit/nsevent?language=objc>
id_class!(NSEvent);

/// <https://developer.apple.com/documentation/appkit/nsresponder?language=objc>
id_class!(NSResponder);

/// <https://developer.apple.com/documentation/appkit/nsview?language=objc>
id_class!(NSView);

/// <https://developer.apple.com/documentation/appkit/nsapplication?language=objc>
id_class!(NSApplication);

/// <https://developer.apple.com/documentation/appkit/nsapplicationdelegate?language=objc>
id_protocol!(NSApplicationDelegate);

/// <https://developer.apple.com/documentation/appkit/nswindow?language=objc>
id_class!(NSWindow);

/// <https://developer.apple.com/documentation/appkit/nswindowdelegate?language=objc>
id_protocol!(NSWindowDelegate);

/// <https://developer.apple.com/documentation/appkit/nsscreen?language=objc>
id_class!(NSScreen);

/// <https://developer.apple.com/documentation/foundation/nsarray?language=objc>
#[repr(transparent)]
pub struct NSArray<T>(pub Object, ::core::marker::PhantomData<T>);

/// <https://developer.apple.com/documentation/objectivec/nsinteger?language=objc>
pub use ::cocoa::foundation::NSInteger;

/// <https://developer.apple.com/documentation/objectivec/nsuinteger?language=objc>
pub use ::cocoa::foundation::NSUInteger;

/// <https://developer.apple.com/documentation/foundation/nstimeinterval?language=objc>
pub use ::cocoa::foundation::NSTimeInterval;

/// <https://developer.apple.com/documentation/foundation/nspoint?language=objc>
pub use ::cocoa::foundation::NSPoint;

/// <https://developer.apple.com/documentation/foundation/nssize?language=objc>
pub use ::cocoa::foundation::NSSize;

/// <https://developer.apple.com/documentation/foundation/nsrect?language=objc>
pub use ::cocoa::foundation::NSRect;

/// <https://developer.apple.com/documentation/corefoundation/cgfloat?language=objc>
pub use ::cocoa::appkit::CGFloat;

/// <https://developer.apple.com/documentation/appkit/nsapplicationactivationpolicy?language=objc>
pub use ::cocoa::appkit::NSApplicationActivationPolicy;

/// <https://developer.apple.com/documentation/appkit/nsapplicationterminatereply?language=objc>
pub use ::cocoa::appkit::NSApplicationTerminateReply;

/// <https://developer.apple.com/documentation/appkit/nswindowstylemask?language=objc>
pub use ::cocoa::appkit::NSWindowStyleMask;

/// <https://developer.apple.com/documentation/appkit/nsbackingstoretype?language=objc>
pub use ::cocoa::appkit::NSBackingStoreType;

// ================================================================================================================================ //
// Functions
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/foundation/nsstring?language=objc>
pub mod ns_string {
    use super::*;

    /// <https://developer.apple.com/documentation/foundation/nsstring/1497379-stringwithutf8string?language=objc>
    pub unsafe fn stringWithUTF8String_(nullTerminatedCString: *const c_char) -> *mut NSString {
        msg_send![
            class!(NSString),
            stringWithUTF8String: nullTerminatedCString
        ]
    }

    /// <https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string?language=objc>
    pub unsafe fn UTF8String(this: *mut NSString) -> *const c_char {
        msg_send![this, UTF8String]
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/foundation/nsthread?language=objc>
pub mod ns_thread {
    use super::*;

    /// <https://developer.apple.com/documentation/foundation/nsthread/1412704-ismainthread?language=objc>
    pub unsafe fn isMainThread_class() -> BOOL {
        msg_send![class!(NSThread), isMainThread]
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/foundation/nsnotification?language=objc>
pub mod ns_notification {
    use super::*;

    /// <https://developer.apple.com/documentation/foundation/nsnotification/1414469-object?language=objc>
    pub unsafe fn object(this: *mut NSNotification) -> id {
        msg_send![this, object]
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/appkit/nsevent?language=objc>
pub mod ns_event {
    use super::*;

    /// <https://developer.apple.com/documentation/appkit/nsevent/1526044-startperiodiceventsafterdelay/>
    pub unsafe fn startPeriodicEventsAfterDelay_withPeriod_(
        delay: NSTimeInterval,
        period: NSTimeInterval,
    ) {
        msg_send![
            class!(NSEvent),
            startPeriodicEventsAfterDelay: delay
            withPeriod: period
        ]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1533746-stopperiodicevents?language=objc>
    pub unsafe fn stopPeriodicEvents() {
        msg_send![class!(NSEvent), stopPeriodicEvents]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1530808-window?language=objc>
    pub unsafe fn window(this: *mut NSEvent) -> *mut NSWindow {
        msg_send![this, window]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1533380-mouselocation?language=objc>
    pub unsafe fn mouseLocation() -> NSPoint {
        msg_send![class!(NSEvent), mouseLocation]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1529068-locationinwindow?language=objc>
    pub unsafe fn locationInWindow(this: *mut NSEvent) -> NSPoint {
        msg_send![this, locationInWindow]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1524505-scrollingdeltax?language=objc>
    pub unsafe fn scrollingDeltaX(this: *mut NSEvent) -> CGFloat {
        msg_send![this, scrollingDeltaX]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1535387-scrollingdeltay?language=objc>
    pub unsafe fn scrollingDeltaY(this: *mut NSEvent) -> CGFloat {
        msg_send![this, scrollingDeltaY]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1527828-buttonnumber?language=objc>
    pub unsafe fn buttonNumber(this: *mut NSEvent) -> NSInteger {
        msg_send![this, buttonNumber]
    }

    /// <https://developer.apple.com/documentation/appkit/nsevent/1534513-keycode?language=objc>
    pub unsafe fn keyCode(this: *mut NSEvent) -> c_ushort {
        msg_send![this, keyCode]
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/appkit/nsapplication?language=objc>
pub mod ns_application {
    use super::*;

    /// <https://developer.apple.com/documentation/appkit/nsapplication/1428360-sharedapplication?language=objc>
    pub unsafe fn sharedApplication() -> *mut NSApplication {
        msg_send![class!(NSApplication), sharedApplication]
    }

    /// <https://developer.apple.com/documentation/appkit/nsapplication/1428705-delegate?language=objc>
    pub unsafe fn setDelegate_(this: *mut NSApplication, delegate: *mut NSApplicationDelegate) {
        msg_send![this, setDelegate: delegate]
    }

    /// <https://developer.apple.com/documentation/appkit/nsapplication/1428631-run?language=objc>
    pub unsafe fn run(this: *mut NSApplication) {
        msg_send![this, run]
    }

    /// <https://developer.apple.com/documentation/appkit/nsapplication/1428473-stop?language=objc>
    pub unsafe fn stop_(this: *mut NSApplication, sender: id) {
        msg_send![this, stop: sender]
    }

    /// <https://developer.apple.com/documentation/appkit/nsapplication/1428759-running?language=objc>
    pub unsafe fn running(this: *mut NSApplication) -> BOOL {
        msg_send![this, isRunning]
    }

    /// <https://developer.apple.com/documentation/appkit/nsapplication/1428621-setactivationpolicy?language=objc>
    pub unsafe fn setActivationPolicy_(
        this: *mut NSApplication,
        activationPolicy: NSApplicationActivationPolicy,
    ) -> BOOL {
        msg_send![this, setActivationPolicy: activationPolicy]
    }

    /// <https://developer.apple.com/documentation/appkit/nsapplication/1428468-activateignoringotherapps?language=objc>
    pub unsafe fn activateIgnoringOtherApps_(this: *mut NSApplication, flag: BOOL) {
        msg_send![this, activateIgnoringOtherApps: flag]
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://developer.apple.com/documentation/appkit/nswindow?language=objc>
pub mod ns_window {
    use super::*;

    /// <https://developer.apple.com/documentation/objectivec/nsobject/1571958-alloc/>
    pub unsafe fn alloc() -> *mut NSWindow {
        msg_send![class!(NSWindow), alloc]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419477-initwithcontentrect?language=objc>
    pub unsafe fn initWithContentRect_styleMask_backing_defer_(
        this: *mut NSWindow,
        contentRect: NSRect,
        style: NSWindowStyleMask,
        backingStoreType: NSBackingStoreType,
        flag: BOOL,
    ) -> *mut NSWindow {
        msg_send![this, initWithContentRect:contentRect styleMask:style backing:backingStoreType defer:flag]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419060-delegate?language=objc>
    pub unsafe fn setDelegate_(this: *mut NSWindow, delegate: *mut NSWindowDelegate) {
        msg_send![this, setDelegate: delegate]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419366-makefirstresponder?language=objc>
    pub unsafe fn makeFirstResponder_(this: *mut NSWindow, responder: *mut NSResponder) -> BOOL {
        msg_send![this, makeFirstResponder: responder]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419479-initialfirstresponder?language=objc>
    pub unsafe fn setInitialFirstResponder_(
        this: *mut NSWindow,
        initialFirstResponder: *mut NSResponder,
    ) {
        msg_send![this, setInitialFirstResponder: initialFirstResponder]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419340-acceptsmousemovedevents?language=objc>
    pub unsafe fn setAcceptsMouseMovedEvents_(this: *mut NSWindow, acceptsMouseMovedEvents: BOOL) {
        msg_send![this, setAcceptsMouseMovedEvents: acceptsMouseMovedEvents]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419160-contentview?language=objc>
    pub unsafe fn contentView(this: *mut NSWindow) -> *mut NSView {
        msg_send![this, contentView]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419160-contentview?language=objc>
    pub unsafe fn setContentView_(this: *mut NSWindow, contentView: *mut NSView) {
        msg_send![this, setContentView: contentView]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419288-performclose?language=objc>
    pub unsafe fn performClose_(this: *mut NSWindow, sender: id) {
        msg_send![this, performClose: sender]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419368-makekeywindow?language=objc>
    pub unsafe fn makeKeyWindow(this: *mut NSWindow) {
        msg_send![this, makeKeyWindow]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419208-makekeyandorderfront?language=objc>
    pub unsafe fn makeKeyAndOrderFront_(this: *mut NSWindow, sender: id) {
        msg_send![this, makeKeyAndOrderFront: sender]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419495-orderfront?language=objc>
    pub unsafe fn orderFront_(this: *mut NSWindow, sender: id) {
        msg_send![this, orderFront: sender]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419495-orderout?language=objc>
    pub unsafe fn orderOut_(this: *mut NSWindow, sender: id) {
        msg_send![this, orderOut: sender]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419404-title?language=objc>
    pub unsafe fn title(this: *mut NSWindow) -> *mut NSString {
        msg_send![this, title]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419404-title?language=objc>
    pub unsafe fn setTitle_(this: *mut NSWindow, title: *mut NSString) {
        msg_send![this, setTitle: title]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419090-center?language=objc>
    pub unsafe fn center(this: *mut NSWindow) {
        msg_send![this, center]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419697-frame?language=objc>
    pub unsafe fn frame(this: *mut NSWindow) -> NSRect {
        msg_send![this, frame]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419124-contentlayoutrect?language=objc>
    pub unsafe fn contentLayoutRect(this: *mut NSWindow) -> NSRect {
        msg_send![this, contentLayoutRect]
    }

    /// `- (NSRect)contentRectForFrameRect:(NSRect)frameRect`
    /// <https://developer.apple.com/documentation/appkit/nswindow/1419108-contentrectforframerect?language=objc>
    pub unsafe fn contentRectForFrameRect_(this: *mut NSWindow, frameRect: NSRect) -> NSRect {
        msg_send![this, contentRectForFrameRect: frameRect]
    }

    /// `- (NSRect)frameRectForContentRect:(NSRect)contentRect`
    /// <https://developer.apple.com/documentation/appkit/nswindow/1419134-framerectforcontentrect?language=objc>
    pub unsafe fn frameRectForContentRect_(this: *mut NSWindow, contentRect: NSRect) -> NSRect {
        msg_send![this, frameRectForContentRect: contentRect]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/2967182-convertpointtoscreen?language=objc>
    pub unsafe fn convertPointToScreen_(this: *mut NSWindow, point: NSPoint) -> NSPoint {
        msg_send![this, convertPointToScreen: point]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419690-setframeorigin?language=objc>
    pub unsafe fn setFrameOrigin_(this: *mut NSWindow, point: NSPoint) {
        msg_send![this, setFrameOrigin: point]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419753-setframe?language=objc>
    pub unsafe fn setFrame_display_(this: *mut NSWindow, frameRect: NSRect, display: BOOL) {
        msg_send![this, setFrame: frameRect display: display]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419100-setcontentsize?language=objc>
    pub unsafe fn setContentSize_(this: *mut NSWindow, size: NSSize) {
        msg_send![this, setContentSize: size]
    }

    /// <https://developer.apple.com/documentation/appkit/nswindow/1419280-mouselocationoutsideofeventstrea?language=objc>
    pub unsafe fn mouseLocationOutsideOfEventStream(this: *mut NSWindow) -> NSPoint {
        msg_send![this, mouseLocationOutsideOfEventStream]
    }
}

/// <https://developer.apple.com/documentation/appkit/nsscreen?language=objc>
pub mod ns_screen {
    use super::*;

    /// <https://developer.apple.com/documentation/appkit/nsscreen/1388371-mainscreen?language=objc>
    pub unsafe fn mainScreen() -> *mut NSScreen {
        msg_send![class!(NSScreen), mainScreen]
    }

    /// <https://developer.apple.com/documentation/appkit/nsscreen/1388393-screens?language=objc>
    pub unsafe fn screens() -> *mut NSArray<*mut NSScreen> {
        msg_send![class!(NSScreen), screens]
    }

    /// <https://developer.apple.com/documentation/appkit/nsscreen/1388387-frame?language=objc>
    pub unsafe fn frame(this: *mut NSScreen) -> NSRect {
        msg_send![this, frame]
    }

    /// <https://developer.apple.com/documentation/appkit/nsscreen/3228043-localizedname?language=objc>
    pub unsafe fn localizedName(this: *mut NSScreen) -> *mut NSString {
        msg_send![this, localizedName]
    }
}

/// <https://developer.apple.com/documentation/foundation/nsarray?language=objc>
pub mod ns_array {
    use super::*;

    /// <https://developer.apple.com/documentation/foundation/nsarray/1409982-count?language=objc>
    pub unsafe fn count<T>(this: *mut NSArray<T>) -> u64 {
        msg_send![this as id, count]
    }

    /// <https://developer.apple.com/documentation/foundation/nsarray/1417555-objectatindex?language=objc>
    pub unsafe fn objectAtIndex<T: 'static>(this: *mut NSArray<T>, index: NSUInteger) -> T {
        msg_send![this as id, objectAtIndex: index]
    }
}

// ================================================================================================================================ //
// Constants
// -------------------------------------------------------------------------------------------------------------------------------- //

// ================================================================================================================================ //
