/*
 *  Crate: Wyn
 * Module: Win32 - Errors
 */

//! Functionality for handling Error Codes and Error Messages.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

use std::error::Error;
use std::os::windows::prelude::OsStringExt;
use std::sync::atomic::AtomicBool;

// ================================================================================================================================ //

/// The `panic` that was caught to cross FFI-boundaries.\
/// Lifetime is tied to an `EventLoopGuard` object in the `EventLoop::run` function.\
/// ## SAFETY
/// Should only ever be accessed by the Event Thread, and only while the EVENTLOOP_MUTEX is acquired by the current thread.
static mut PANIC: Option<BoxedPanic> = None;

/// Flag to indicate a Panic is occuring to other threads.
static PANIC_FLAG: AtomicBool = AtomicBool::new(false);

/// Returns whether or not there was a Panic stored.
/// ## SAFETY
/// Must be called on the Event Thread.
pub(crate) unsafe fn internal_is_panicking() -> bool {
    PANIC.is_some()
}

/// Returns whether or not there was a Panic stored.
pub(crate) fn is_panicking() -> bool {
    PANIC_FLAG.load(std::sync::atomic::Ordering::Acquire)
}

/// Stores a `panic` to carry across FFI-boundaries.\
/// If a second `panic` is attempted to be stored while a previous one is stored, the process will abort.
/// ## SAFETY
/// Should only ever be called by the Event Thread, and only while the Event Thread lock is held by current thread.
pub(crate) unsafe fn store_panic(err: BoxedPanic) {
    match &mut PANIC {
        // There was not a previous `panic`, so store this one and tell the Event Loop to shut down.
        None => {
            let _ = PANIC.insert(err);
            PANIC_FLAG.store(true, std::sync::atomic::Ordering::Release);

            if let Some(events) = event_loop::EVENTLOOP {
                events.request_stop();
            }
        }
        // There was already a previous `panic`, so abort the process.
        Some(_err) => std::process::abort(),
    };
}

/// Resumes a `panic` that was caught earlier, if there is one.
/// ## SAFETY
/// Should only ever be called by the Event Thread, and only while the  Event Thread lock is held by current thread.
pub(crate) unsafe fn resume_if_panicking() {
    if let Some(err) = PANIC.take() {
        std::panic::resume_unwind(err);
    }
}

// ================================================================================================================================ //

/// Converts integer results from Win32 API functions into Result types, checking whether or not a zero value is truly an error.
macro_rules! sys_verify {
    ($expr:expr) => {{
        #[allow(unused_unsafe)]
        let res = unsafe { $expr };

        if res == 0 {
            match crate::errors::WinError::current() {
                None => Ok(res),
                Some(err) => Err(err),
            }
        } else {
            Ok(res)
        }
    }};
}

// ================================================================================================================================ //

/// Win32 Error Code.
pub type WinErrorCode = sys::WIN32_ERROR;

/// Nonzero wrapper for Win32 Error Codes.
type NonzeroWinErrorCode = NonZero<WinErrorCode>;

/// Wrapper type for Win32 Errors.
#[repr(transparent)]
pub struct WinError(NonzeroWinErrorCode);

/// A `Result` where the error type is a `WinError`.
pub type WinResult<T> = Result<T, WinError>;

// ---------------------------------------------------------------- //

impl WinError {
    /// Creates a WinError from a WinErrorCode.
    pub fn new(code: WinErrorCode) -> Option<Self> {
        NonzeroWinErrorCode::new(code).map(Self)
    }

    /// Retrieves the current thread's Win32 Error.
    pub(crate) fn current() -> Option<Self> {
        // SAFETY: This function cannot fail.
        let code = unsafe { sys::GetLastError() };
        NonzeroWinErrorCode::new(code).map(Self)
    }

    /// Clears the current thread's Win32 Error.
    #[allow(unused)]
    pub(crate) fn clear() {
        // SAFETY: This function cannot fail.
        unsafe { sys::SetLastError(0) };
    }
}

// ---------------------------------------------------------------- //

impl WinError {
    /// The Error-Code associated with this Error.
    pub fn code(&self) -> WinErrorCode {
        self.0.get()
    }

    /// The Error-Message associated with this Error.
    pub fn message(&self) -> String {
        let msg = WinErrorMessage::new(self.code());
        let os_txt = msg.os_string();
        os_txt.to_string_lossy().trim().to_string()
    }
}

// ---------------------------------------------------------------- //

impl Error for WinError {}

impl Debug for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        let msg = self.message();
        write!(f, "Win32 Error [0x{code:X}] \"{msg}\"")
    }
}

impl Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        let msg = self.message();
        write!(f, "Win32 Error [0x{code:X}] \"{msg}\"")
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Native handle to a Win32 Error Message.
type NativeWinErrorMessage = sys::HLOCAL;

/// Nonzero handle to a Win32 Error Message.
type NonzeroWinErrorMessage = NonZero<NativeWinErrorMessage>;

/// An Error Message for Win32 Errors.
struct WinErrorMessage {
    /// Pointer to the message data.
    data: NonzeroWinErrorMessage,

    /// Length of the message data in characters.
    len: u32,
}

// ---------------------------------------------------------------- //

impl WinErrorMessage {
    /// Retrieves a new Error Message for the given Error Code.
    fn new(code: WinErrorCode) -> Self {
        let mut message_ptr: sys::HLOCAL = 0;

        let dwflags = sys::FORMAT_MESSAGE_FROM_SYSTEM
            | sys::FORMAT_MESSAGE_ALLOCATE_BUFFER
            | sys::FORMAT_MESSAGE_IGNORE_INSERTS;

        // <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#:~:text=If%20neither%20of%20these%20flags%20is%20set%20in%20dwFlags%2C%20then%20lpSource%20is%20ignored>
        let lpsource = null();

        // <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#:~:text=If%20this%20flag%20is%20specified%2C%20an%20application%20can%20pass%20the%20result%20of%20the%20GetLastError%20function%20to%20retrieve%20the%20message%20text%20for%20a%20system%2Ddefined%20error>
        let dwmessageid = code;

        // <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#:~:text=If%20you%20pass%20in%20zero%2C%20FormatMessage%20looks%20for%20a%20message%20for%20LANGIDs%20in%20the%20following%20order%3A>
        let dwlanguageid = 0;

        // SAFETY: Because we pass in `FORMAT_MESSAGE_ALLOCATE_BUFFER`, `lpbuffer` must instead be
        // an address of a pointer to be overwritten, as opposed to a pointer to a character-buffer.
        // <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#:~:text=The%20lpBuffer%20parameter%20is%20a%20pointer%20to%20an%20LPTSTR%3B%20you%20must%20cast%20the%20pointer%20to%20an%20LPTSTR%20(for%20example%2C%20(LPTSTR)%26lpBuffer)>
        let lpbuffer = unsafe { transmute(addr_of_mut!(message_ptr)) };

        // <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#:~:text=If%20FORMAT_MESSAGE_ALLOCATE_BUFFER%20is%20set%2C%20this%20parameter%20specifies%20the%20minimum%20number%20of%20TCHARs%20to%20allocate%20for%20an%20output%20buffer>
        let nsize = 0;

        // <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#:~:text=If%20this%20flag%20is%20set%2C%20the%20Arguments%20parameter%20is%20ignored>
        let arguments = null();

        // <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#return-value>
        // SAFETY: The arguments follow the instructions in the documentation for FormatMessage, as referenced above.
        let res = sys_verify! {
            sys::FormatMessageW(
                dwflags,
                lpsource,
                dwmessageid,
                dwlanguageid,
                lpbuffer,
                nsize,
                arguments,
            )
        };
        let len = res.expect("Win32 Error Message should not fail.");
        let data = NonzeroWinErrorMessage::new(message_ptr)
            .expect("Win32 Error Message should not be NULL.");

        WinErrorMessage { data, len }
    }
}

impl Drop for WinErrorMessage {
    fn drop(&mut self) {
        // SAFETY: `self.data` is guaranted to be non-null, and can only be allocated by `FormatMessage`.
        unsafe { sys::LocalFree(self.data.get()) };
    }
}

// ---------------------------------------------------------------- //

impl WinErrorMessage {
    /// The contents of the message, as a slice of Windows UTF-16 Code Points.
    fn os_slice(&self) -> &[u16] {
        let data = self.data.get() as *const u16;
        let len = self.len as usize;
        // SAFETY: `data` is guaranteed to be Non-Null, with `data` and `len` retrieved directly from the Win32 API.
        unsafe { core::slice::from_raw_parts(data, len) }
    }

    /// The contents of the message, as a Windows OsString.
    fn os_string(&self) -> OsString {
        OsString::from_wide(self.os_slice())
    }
}

// ================================================================================================================================ //
