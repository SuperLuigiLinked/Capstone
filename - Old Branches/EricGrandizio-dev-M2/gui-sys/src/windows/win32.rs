/*
 *  Crate: GUI-Sys
 * Module: Windows - Win32
 */

//! Windows Win32 bindings.
//!
//! # Dependencies
//! * <https://docs.rs/windows-sys/latest/windows_sys/>
//!
//! # Documentation
//! * <https://learn.microsoft.com/en-us/windows/win32/>

// -------------------------------------------------------------------------------------------------------------------------------- //

use crate::common::c_types::*;

pub use ::windows_sys::Win32::Foundation::S_OK;
pub use ::windows_sys::Win32::Graphics::Dwm::DwmEnableBlurBehindWindow;
pub use ::windows_sys::Win32::Graphics::Dwm::DWM_BB_ENABLE;
pub use ::windows_sys::Win32::Graphics::Dwm::DWM_BLURBEHIND;

// ================================================================================================================================ //
// Macros
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://docs.rs/windows-sys/latest/windows_sys/macro.s.html>\
#[macro_export]
pub use ::windows_sys::s;

/// <https://docs.rs/windows-sys/latest/windows_sys/macro.w.html>\
#[macro_export]
pub use ::windows_sys::w;

// -------------------------------------------------------------------------------------------------------------------------------- //

// Macro-Definitions taken from `<windowsx.h>`

/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85)>
pub const fn LOBYTE(wValue: WORD) -> BYTE {
    // #define LOBYTE(w) ((BYTE)(((DWORD_PTR)(w)) & 0xff))
    ((wValue as DWORD_PTR) & 0xFF) as BYTE
}

/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632656(v=vs.85)>
pub const fn HIBYTE(wValue: WORD) -> BYTE {
    // #define HIBYTE(w) ((BYTE)((((DWORD_PTR)(w)) >> 8) & 0xff))
    (((wValue as DWORD_PTR) >> 8) & 0xFF) as BYTE
}

/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632659(v=vs.85)>
pub const fn LOWORD(dwValue: DWORD) -> WORD {
    // #define LOWORD(l) ((WORD)(((DWORD_PTR)(l)) & 0xffff))
    ((dwValue as DWORD_PTR) & 0xFFFF) as WORD
}

/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632657(v=vs.85)>
pub const fn HIWORD(dwValue: DWORD) -> WORD {
    // #define HIWORD(l) ((WORD)((((DWORD_PTR)(l)) >> 16) & 0xffff))
    (((dwValue as DWORD_PTR) >> 16) & 0xFFFF) as WORD
}

/// <https://learn.microsoft.com/en-us/windows/win32/api/windowsx/nf-windowsx-get_x_lparam>
pub const fn GET_X_LPARAM(lp: LPARAM) -> i32 {
    // #define GET_X_LPARAM(lp) ((int)(short)LOWORD(lp))
    LOWORD(lp as DWORD) as i16 as i32
}

/// <https://learn.microsoft.com/en-us/windows/win32/api/windowsx/nf-windowsx-get_y_lparam>
pub const fn GET_Y_LPARAM(lp: LPARAM) -> i32 {
    // #define GET_Y_LPARAM(lp) ((int)(short)HIWORD(lp))
    HIWORD(lp as DWORD) as i16 as i32
}

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-get_keystate_wparam>
pub const fn GET_KEYSTATE_WPARAM(wParam: WPARAM) -> WORD {
    // #define GET_KEYSTATE_WPARAM(wParam) (LOWORD(wParam))
    LOWORD(wParam as DWORD)
}

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-get_wheel_delta_wparam>
pub const fn GET_WHEEL_DELTA_WPARAM(wParam: WPARAM) -> i16 {
    // #define GET_WHEEL_DELTA_WPARAM(wParam) ((short)HIWORD(wParam))
    HIWORD(wParam as DWORD) as i16
}

// ================================================================================================================================ //
// Types
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL>
pub use ::windows_sys::Win32::Foundation::BOOL;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BYTE>
pub type BYTE = u8;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#WORD>
pub type WORD = u16;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#DWORD>
pub type DWORD = u32;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#QWORD>
pub type QWORD = u64;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#UINT>
pub type UINT = u32;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#LONG>
pub type LONG = c_long;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#LONG_PTR>
pub type LONG_PTR = isize;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#ULONG_PTR>
pub type ULONG_PTR = usize;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#DWORD_PTR>
pub type DWORD_PTR = ULONG_PTR;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#PCSTR>
pub use ::windows_sys::core::PCSTR;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#PCWSTR>
pub use ::windows_sys::core::PCWSTR;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#WPARAM>
pub use ::windows_sys::Win32::Foundation::WPARAM;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#LPARAM>
pub use ::windows_sys::Win32::Foundation::LPARAM;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#LRESULT>
pub use ::windows_sys::Win32::Foundation::LRESULT;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HANDLE>
pub use ::windows_sys::Win32::Foundation::HANDLE;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HINSTANCE>
pub type HINSTANCE = HANDLE;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HWND>
pub use ::windows_sys::Win32::Foundation::HWND;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HDC>
pub use ::windows_sys::Win32::Graphics::Gdi::HDC;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HMONITOR>
pub use ::windows_sys::Win32::Graphics::Gdi::HMONITOR;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HBRUSH>
pub use ::windows_sys::Win32::Graphics::Gdi::HBRUSH;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HHOOK>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::HHOOK;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#HLOCAL>
pub type HLOCAL = HANDLE;

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#ATOM>
pub type ATOM = WORD;

/// <https://learn.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point>\
pub use ::windows_sys::Win32::Foundation::POINT;

/// <https://learn.microsoft.com/en-us/windows/win32/api/windef/ns-windef-size>\
pub use ::windows_sys::Win32::Foundation::SIZE;

/// <https://learn.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect>\
pub use ::windows_sys::Win32::Foundation::RECT;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::MSG;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassa>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexa>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSEXA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WNDPROC;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowpos>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WINDOWPOS;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-hookproc>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::HOOKPROC;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-monitorenumproc>\
pub use ::windows_sys::Win32::Graphics::Gdi::MONITORENUMPROC;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-monitorinfo>\
pub use ::windows_sys::Win32::Graphics::Gdi::MONITORINFO;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-monitorinfoexa>\
pub use ::windows_sys::Win32::Graphics::Gdi::MONITORINFOEXA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-monitorinfoexw>\
pub use ::windows_sys::Win32::Graphics::Gdi::MONITORINFOEXW;

/// <https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes>\
pub use ::windows_sys::Win32::Foundation::WIN32_ERROR;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASS_STYLES;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#parameters>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WINDOW_LONG_PTR_INDEX;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#parameters>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#parameters>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SET_WINDOW_POS_FLAGS;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#parameters>\
pub use ::windows_sys::Win32::Graphics::Gdi::REDRAW_WINDOW_FLAGS;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw?source=recommendations#parameters>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WINDOWS_HOOK_ID;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#windows-1011-system-colors>
pub use ::windows_sys::Win32::Graphics::Gdi::SYS_COLOR_INDEX;

/// <https://learn.microsoft.com/en-us/windows/win32/learnwin32/mouse-clicks#additional-flags>\
pub use ::windows_sys::Win32::System::SystemServices::MODIFIERKEYS_FLAGS;

/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_state>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_STATE;

/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD;

/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#members>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_BUTTON_FLAGS;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct>\
pub use ::windows_sys::Win32::Graphics::Gdi::PAINTSTRUCT;

/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/ns-timeapi-timecaps>
pub use ::windows_sys::Win32::Media::TIMECAPS;

// ================================================================================================================================ //
// Functions
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlea>\
pub use ::windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew>\
pub use ::windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagea>\
pub use ::windows_sys::Win32::System::Diagnostics::Debug::FormatMessageA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew>\
pub use ::windows_sys::Win32::System::Diagnostics::Debug::FormatMessageW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree>\
pub use ::windows_sys::Win32::System::Memory::LocalFree;

/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid>\
pub use ::windows_sys::Win32::System::Threading::GetCurrentThreadId;

/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid>\
pub use ::windows_sys::Win32::System::Threading::GetCurrentProcessId;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindows>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::EnumWindows;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumthreadwindows>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::EnumThreadWindows;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptra>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptra>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongPtrA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetClientRect;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-logicaltophysicalpoint>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::LogicalToPhysicalPoint;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-physicaltologicalpoint>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::PhysicalToLogicalPoint;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient>\
pub use ::windows_sys::Win32::Graphics::Gdi::ScreenToClient;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen>\
pub use ::windows_sys::Win32::Graphics::Gdi::ClientToScreen;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrect>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::AdjustWindowRect;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::AdjustWindowRectEx;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectexfordpi>\
pub use ::windows_sys::Win32::UI::HiDpi::AdjustWindowRectExForDpi;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdpiforwindow>\
pub use ::windows_sys::Win32::UI::HiDpi::GetDpiForWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagea>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetMessageA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetMessageW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagea>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::DispatchMessageA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::DispatchMessageW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::TranslateMessage;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowproca>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::DefWindowProcA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::DefWindowProcW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror>\
pub use ::windows_sys::Win32::Foundation::GetLastError;

/// <https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror>\
pub use ::windows_sys::Win32::Foundation::SetLastError;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagea>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SendMessageA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SendMessageW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagea>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::PostThreadMessageA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagew>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::PostThreadMessageW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::PostQuitMessage;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassa>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::RegisterClassA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::RegisterClassW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexa>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::RegisterClassExA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::RegisterClassExW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassa>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::UnregisterClassA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::UnregisterClassW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::LoadCursorA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::LoadCursorW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CreateWindowExA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CreateWindowExW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::DestroyWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::ShowWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindowasync>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::ShowWindowAsync;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetWindowPos;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setlayeredwindowattributes>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetLayeredWindowAttributes;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow>\
pub use ::windows_sys::Win32::Graphics::Gdi::RedrawWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect>\
pub use ::windows_sys::Win32::Graphics::Gdi::InvalidateRect;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect>\
pub use ::windows_sys::Win32::Graphics::Gdi::ValidateRect;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexa>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetWindowsHookExA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetWindowsHookExW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwindowshookex>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CallNextHookEx;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfoa>\
pub use ::windows_sys::Win32::Graphics::Gdi::GetMonitorInfoA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow>\
pub use ::windows_sys::Win32::Graphics::Gdi::GetMonitorInfoW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaymonitors>\
pub use ::windows_sys::Win32::Graphics::Gdi::EnumDisplayMonitors;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint>\
pub use ::windows_sys::Win32::Graphics::Gdi::MonitorFromPoint;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromwindow>\
pub use ::windows_sys::Win32::Graphics::Gdi::MonitorFromWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtexta>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetWindowTextA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetWindowTextW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtexta>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowTextA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowTextW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengtha>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowTextLengthA;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetWindowTextLengthW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setactivewindow>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::SetActiveWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IsWindowVisible;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IsIconic;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IsZoomed;

/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XInputGetState;

/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XInputSetState;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint>\
pub use ::windows_sys::Win32::Graphics::Gdi::BeginPaint;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint>\
pub use ::windows_sys::Win32::Graphics::Gdi::EndPaint;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc>\
pub use ::windows_sys::Win32::Graphics::Gdi::GetDC;

/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod>\
pub use ::windows_sys::Win32::Media::timeBeginPeriod;

/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timeendperiod>\
pub use ::windows_sys::Win32::Media::timeEndPeriod;

/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timegetdevcaps>\
pub use ::windows_sys::Win32::Media::timeGetDevCaps;

/// <https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-swapbuffers>
pub use ::windows_sys::Win32::Graphics::OpenGL::SwapBuffers;

// ================================================================================================================================ //
// Constants
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL>
pub use ::windows_sys::Win32::Foundation::FALSE;
/// <https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL>
pub use ::windows_sys::Win32::Foundation::TRUE;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage#FORMAT_MESSAGE_ALLOCATE_BUFFER>\
pub use ::windows_sys::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_ALLOCATE_BUFFER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage#FORMAT_MESSAGE_FROM_SYSTEM>\
pub use ::windows_sys::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_FROM_SYSTEM;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage#FORMAT_MESSAGE_IGNORE_INSERTS>\
pub use ::windows_sys::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_IGNORE_INSERTS;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_BYTEALIGNCLIENT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_BYTEALIGNCLIENT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_BYTEALIGNWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_BYTEALIGNWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_CLASSDC>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_CLASSDC;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_DBLCLKS>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_DBLCLKS;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_DROPSHADOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_DROPSHADOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_GLOBALCLASS>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_GLOBALCLASS;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_HREDRAW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_HREDRAW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_NOCLOSE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_NOCLOSE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_OWNDC>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_OWNDC;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_PARENTDC>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_PARENTDC;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_SAVEBITS>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_SAVEBITS;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#CS_VREDRAW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CS_VREDRAW;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_ACTIVECAPTION>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_ACTIVECAPTION;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_BORDER>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_BORDER;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_CAPTION>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_CAPTION;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_CHILD>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_CHILD;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_CHILDWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_CHILDWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_CLIPCHILDREN>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_CLIPCHILDREN;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_CLIPSIBLINGS>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_CLIPSIBLINGS;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_DISABLED>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_DISABLED;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_DLGFRAME>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_DLGFRAME;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_GROUP>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_GROUP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_HSCROLL>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_HSCROLL;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_ICONIC>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_ICONIC;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_MAXIMIZE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_MAXIMIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_MAXIMIZEBOX>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_MAXIMIZEBOX;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_MINIMIZE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_MINIMIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_MINIMIZEBOX>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_MINIMIZEBOX;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_OVERLAPPED>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_OVERLAPPED;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_OVERLAPPEDWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_OVERLAPPEDWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_POPUP>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_POPUP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_POPUPWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_POPUPWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_SIZEBOX>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_SIZEBOX;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_SYSMENU>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_SYSMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_TABSTOP>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_TABSTOP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_THICKFRAME>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_THICKFRAME;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_TILED>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_TILED;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_TILEDWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_TILEDWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_VISIBLE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_VISIBLE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles#WS_VSCROLL>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_VSCROLL;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_ACCEPTFILES>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_ACCEPTFILES;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_APPWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_APPWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_CLIENTEDGE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_CLIENTEDGE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_COMPOSITED>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_COMPOSITED;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_CONTEXTHELP>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_CONTEXTHELP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_CONTROLPARENT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_CONTROLPARENT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_DLGMODALFRAME>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_DLGMODALFRAME;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_LAYERED>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_LAYERED;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_LAYOUTRTL>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_LAYOUTRTL;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_LEFT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_LEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_LEFTSCROLLBAR>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_LEFTSCROLLBAR;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_LTRREADING>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_LTRREADING;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_MDICHILD>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_MDICHILD;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_NOACTIVATE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_NOACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_NOINHERITLAYOUT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_NOINHERITLAYOUT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_NOPARENTNOTIFY>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_NOPARENTNOTIFY;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_NOREDIRECTIONBITMAP>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_NOREDIRECTIONBITMAP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_OVERLAPPEDWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_OVERLAPPEDWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_PALETTEWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_PALETTEWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_RIGHT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_RIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_RIGHTSCROLLBAR>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_RIGHTSCROLLBAR;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_RTLREADING>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_RTLREADING;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_STATICEDGE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_STATICEDGE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_TOOLWINDOW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_TOPMOST>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_TOPMOST;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_TRANSPARENT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_TRANSPARENT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles#WS_EX_WINDOWEDGE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WS_EX_WINDOWEDGE;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_APPSTARTING>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_APPSTARTING;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_ARROW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_ARROW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_CROSS>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_CROSS;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_HAND>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_HAND;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_HELP>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_HELP;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_IBEAM>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_IBEAM;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_ICON>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_ICON;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_NO>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_NO;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_PERSON>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_PERSON;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_PIN>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_PIN;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_SIZE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_SIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_SIZEALL>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_SIZEALL;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_SIZENESW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_SIZENESW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_SIZENS>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_SIZENS;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_SIZENWSE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_SIZENWSE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_SIZEWE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_SIZEWE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_UPARROW>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_UPARROW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw#IDC_WAIT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::IDC_WAIT;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw#CW_USEDEFAULT:~:text=is%20set%20to-,CW_USEDEFAULT,-%2C%20the%20system%20selects>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_CALLWNDPROC>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_CALLWNDPROC;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_CALLWNDPROCRET>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_CALLWNDPROCRET;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_CBT>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_CBT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_DEBUG>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_DEBUG;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_FOREGROUNDIDLE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_FOREGROUNDIDLE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_GETMESSAGE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_GETMESSAGE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_JOURNALPLAYBACK>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_JOURNALPLAYBACK;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_JOURNALRECORD>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_JOURNALRECORD;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_KEYBOARD>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_KEYBOARD;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_KEYBOARD_LL>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_KEYBOARD_LL;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_MOUSE>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_MOUSE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_MOUSE_LL>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_MOUSE_LL;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_MSGFILTER>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_MSGFILTER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_SHELL>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_SHELL;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw#WH_SYSMSGFILTER>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WH_SYSMSGFILTER;

/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ACTIVATEAPP;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_AFXFIRST;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_AFXLAST;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-app>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_APP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_APPCOMMAND;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-askcbformatname>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ASKCBFORMATNAME;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-canceljournal>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CANCELJOURNAL;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CANCELMODE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CAPTURECHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-changecbchain>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CHANGECBCHAIN;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-changeuistate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CHANGEUISTATE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CHAR;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-chartoitem>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CHARTOITEM;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CHILDACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-clear>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CLEAR;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-clipboardupdate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CLIPBOARDUPDATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CLOSE;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_COMMAND;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_COMMNOTIFY;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-compacting>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_COMPACTING;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-compareitem>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_COMPAREITEM;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CONTEXTMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-copy>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_COPY;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-copydata>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_COPYDATA;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CREATE;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CTLCOLORBTN;
/// <https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CTLCOLORDLG;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CTLCOLOREDIT;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CTLCOLORLISTBOX;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CTLCOLORMSGBOX;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CTLCOLORSCROLLBAR;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CTLCOLORSTATIC;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-cut>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_CUT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-deadchar>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DEADCHAR;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-deleteitem>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DELETEITEM;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DESTROY;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-destroyclipboard>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DESTROYCLIPBOARD;
/// <https://learn.microsoft.com/en-us/windows/win32/devio/wm-devicechange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DEVICECHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-devmodechange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DEVMODECHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-displaychange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DISPLAYCHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DPICHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged-afterparent>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DPICHANGED_AFTERPARENT;
/// <https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged-beforeparent>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DPICHANGED_BEFOREPARENT;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-drawclipboard>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DRAWCLIPBOARD;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-drawitem>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DRAWITEM;
/// <https://learn.microsoft.com/en-us/windows/win32/shell/wm-dropfiles>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DROPFILES;
/// <https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmcolorizationcolorchanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DWMCOLORIZATIONCOLORCHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmcompositionchanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DWMCOMPOSITIONCHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmncrenderingchanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DWMNCRENDERINGCHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmsendiconiclivepreviewbitmap>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DWMSENDICONICLIVEPREVIEWBITMAP;
/// <https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmsendiconicthumbnail>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DWMSENDICONICTHUMBNAIL;
/// <https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmcolorizationcolorchanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_DWMWINDOWMAXIMIZEDCHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-enable>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ENABLE;
/// <https://learn.microsoft.com/en-us/windows/win32/shutdown/wm-endsession>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ENDSESSION;
/// <https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ENTERIDLE;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ENTERMENULOOP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ENTERSIZEMOVE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ERASEBKGND;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_EXITMENULOOP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_EXITSIZEMOVE;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-fontchange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_FONTCHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/wintouch/wm-gesture>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GESTURE;
/// <https://learn.microsoft.com/en-us/windows/win32/wintouch/wm-gesturenotify>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GESTURENOTIFY;
/// <https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETDLGCODE;
/// <https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-getdpiscaledsize>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETDPISCALEDSIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getfont>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETFONT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-gethotkey>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETHOTKEY;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-geticon>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETICON;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETMINMAXINFO;
/// <https://learn.microsoft.com/en-us/windows/win32/winauto/wm-getobject>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETOBJECT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettext>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETTEXTLENGTH;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_GETTITLEBARINFOEX;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_HANDHELDLAST;
/// <https://learn.microsoft.com/en-us/windows/win32/shell/wm-help>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_HELP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-hotkey>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_HOTKEY;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_HSCROLL;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-hscrollclipboard>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_HSCROLLCLIPBOARD;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_ICONERASEBKGND;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-char>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_CHAR;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-composition>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_COMPOSITION;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-compositionfull>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_COMPOSITIONFULL;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-control>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_CONTROL;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-endcomposition>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_ENDCOMPOSITION;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-keydown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_KEYDOWN;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_KEYLAST;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-keyup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_KEYUP;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-notify>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_NOTIFY;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-request>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_REQUEST;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-select>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_SELECT;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-setcontext>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_SETCONTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/intl/wm-ime-startcomposition>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_IME_STARTCOMPOSITION;
/// <https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_INITDIALOG;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-initmenu>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_INITMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_INITMENUPOPUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-input>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_INPUT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-inputlangchange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_INPUTLANGCHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-inputlangchangerequest>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_INPUTLANGCHANGEREQUEST;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-input-device-change>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_INPUT_DEVICE_CHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_KEYDOWN;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_KEYFIRST;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_KEYLAST;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keyup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_KEYUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_KILLFOCUS;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_LBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_LBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_LBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdiactivate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDIACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdicascade>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDICASCADE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdicreate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDICREATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdidestroy>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDIDESTROY;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdigetactive>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDIGETACTIVE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdiiconarrange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDIICONARRANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdimaximize>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDIMAXIMIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdinext>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDINEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdirefreshmenu>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDIREFRESHMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdirestore>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDIRESTORE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mdisetmenu>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDISETMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-mditile>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MDITILE;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-measureitem>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MEASUREITEM;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menuchar>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MENUCHAR;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menucommand>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MENUCOMMAND;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menudrag>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MENUDRAG;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menugetobject>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MENUGETOBJECT;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MENURBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menuselect>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MENUSELECT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mouseactivate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOUSEACTIVATE;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOUSEFIRST;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehwheel>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOUSEHWHEEL;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOUSELAST;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOUSEMOVE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOUSEWHEEL;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-move>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOVE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_MOVING;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-ncactivate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCCALCSIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCCREATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCDESTROY;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCHITTEST;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nclbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nclbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nclbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncmbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCMBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncmbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCMBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncmbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCMBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncmousehover>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCMOUSEHOVER;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncmouseleave>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCMOUSELEAVE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncmousemove>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCMOUSEMOVE;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCPAINT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-ncpointerdown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCPOINTERDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-ncpointerup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCPOINTERUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-ncpointerupdate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCPOINTERUPDATE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncrbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCRBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncrbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCRBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncrbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCRBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncxbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCXBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncxbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCXBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncxbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NCXBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NEXTDLGCTL;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-nextmenu>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NEXTMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-notify>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NOTIFY;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-notifyformat>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NOTIFYFORMAT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-null>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_NULL;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-paint>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PAINT;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-paintclipboard>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PAINTCLIPBOARD;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PAINTICON;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-palettechanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PALETTECHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-paletteischanging>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PALETTEISCHANGING;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PARENTNOTIFY;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-paste>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PASTE;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PENWINLAST;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointeractivate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointercapturechanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERCAPTURECHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerdevicechange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERDEVICECHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerdeviceinrange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERDEVICEINRANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerdeviceoutofrange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERDEVICEOUTOFRANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerdown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerenter>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERENTER;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerhwheel>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERHWHEEL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerleave>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERLEAVE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerroutedaway>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERROUTEDAWAY;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerroutedreleased>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERROUTEDRELEASED;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerroutedto>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERROUTEDTO;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERUP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerupdate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERUPDATE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-pointerwheel>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POINTERWHEEL;
/// <https://learn.microsoft.com/en-us/windows/win32/power/wm-power>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POWER;
/// <https://learn.microsoft.com/en-us/windows/win32/power/wm-powerbroadcast>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_POWERBROADCAST;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-print>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PRINT;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-printclient>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_PRINTCLIENT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-querydragicon>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_QUERYDRAGICON;
/// <https://learn.microsoft.com/en-us/windows/win32/shutdown/wm-queryendsession>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_QUERYENDSESSION;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-querynewpalette>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_QUERYNEWPALETTE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_QUERYOPEN;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-queryuistate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_QUERYUISTATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-queuesync>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_QUEUESYNC;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-quit>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_QUIT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_RBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_RBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_RBUTTONUP;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-renderallformats>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_RENDERALLFORMATS;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-renderformat>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_RENDERFORMAT;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-setcursor>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETCURSOR;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETFOCUS;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-setfont>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETFONT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sethotkey>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETHOTKEY;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-seticon>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETICON;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-setredraw>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETREDRAW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settext>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settingchange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SETTINGCHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SHOWWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-size>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-sizeclipboard>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SIZECLIPBOARD;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-sizing>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SIZING;
/// <https://learn.microsoft.com/en-us/windows/win32/printdocs/wm-spoolerstatus>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SPOOLERSTATUS;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_STYLECHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_STYLECHANGING;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syncpaint>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SYNCPAINT;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syschar>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SYSCHAR;
/// <https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syscolorchange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SYSCOLORCHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SYSCOMMAND;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SYSDEADCHAR;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SYSKEYDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_SYSKEYUP;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST;
/// <...>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_TABLET_LAST;
/// <https://learn.microsoft.com/en-us/windows/win32/shell/wm-tcard>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_TCARD;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_THEMECHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/sysinfo/wm-timechange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_TIMECHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-timer>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_TIMER;
/// <https://learn.microsoft.com/en-us/windows/win32/wintouch/wm-touchdown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_TOUCH;
/// <https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-touchhittesting>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_TOUCHHITTESTING;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-undo>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_UNDO;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-unichar>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_UNICHAR;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_UNINITMENUPOPUP;
/// <https://learn.microsoft.com/en-us/windows/win32/menurc/wm-updateuistate>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_UPDATEUISTATE;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-user>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_USER;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-userchanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_USERCHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-vkeytoitem>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_VKEYTOITEM;
/// <https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_VSCROLL;
/// <https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-vscrollclipboard>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_VSCROLLCLIPBOARD;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_WINDOWPOSCHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_WINDOWPOSCHANGING;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-wininichange>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_WININICHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/termserv/wm-wtssession-change>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_WTSSESSION_CHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_XBUTTONDBLCLK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_XBUTTONDOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WM_XBUTTONUP;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindowex#SW_ERASE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_ERASE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindowex#SW_INVALIDATE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_INVALIDATE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindowex#SW_SCROLLCHILDREN>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SCROLLCHILDREN;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindowex#SW_SMOOTHSCROLL>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SMOOTHSCROLL;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow#SW_OTHERUNZOOM>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_OTHERUNZOOM;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow#SW_OTHERZOOM>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_OTHERZOOM;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow#SW_PARENTCLOSING>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_PARENTCLOSING;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow#SW_PARENTOPENING>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_PARENTOPENING;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_FORCEMINIMIZE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_FORCEMINIMIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_HIDE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_HIDE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_MAXIMIZE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_MAXIMIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_MINIMIZE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_MINIMIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_NORMAL>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_NORMAL;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_RESTORE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_RESTORE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOW>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOWDEFAULT>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWDEFAULT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOWMAXIMIZED>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWMAXIMIZED;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOWMINIMIZED>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWMINIMIZED;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOWMINNOACTIVE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWMINNOACTIVE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOWNA>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNA;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOWNOACTIVATE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNOACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow#SW_SHOWNORMAL>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_ASYNCWINDOWPOS>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_ASYNCWINDOWPOS;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_DEFERERASE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_DEFERERASE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_DRAWFRAME>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_DRAWFRAME;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_FRAMECHANGED>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_HIDEWINDOW>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_HIDEWINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOACTIVATE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOCOPYBITS>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOCOPYBITS;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOMOVE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOMOVE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOOWNERZORDER>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOOWNERZORDER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOREDRAW>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOREDRAW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOREPOSITION>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOREPOSITION;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOSENDCHANGING>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOSENDCHANGING;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOSIZE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOSIZE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_NOZORDER>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOZORDER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#SWP_SHOWWINDOW>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::SWP_SHOWWINDOW;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setlayeredwindowattributes#LWA_ALPHA>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::LWA_ALPHA;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setlayeredwindowattributes#LWA_COLORKEY>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::LWA_COLORKEY;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_ALLCHILDREN>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_ALLCHILDREN;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_ERASE>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_ERASE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_ERASENOW>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_ERASENOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_FRAME>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_FRAME;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_INTERNALPAINT>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_INTERNALPAINT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_INVALIDATE>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_INVALIDATE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_NOCHILDREN>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_NOCHILDREN;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_NOERASE>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_NOERASE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_NOFRAME>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_NOFRAME;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_NOINTERNALPAINT>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_NOINTERNALPAINT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_UPDATENOW>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_UPDATENOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow#RDW_VALIDATE>
pub use ::windows_sys::Win32::Graphics::Gdi::RDW_VALIDATE;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWLP_HINSTANCE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWLP_HINSTANCE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWLP_HWNDPARENT>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWLP_HWNDPARENT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWLP_ID>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWLP_ID;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWLP_USERDATA>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWLP_USERDATA;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWLP_WNDPROC>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWLP_WNDPROC;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWL_EXSTYLE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWL_EXSTYLE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWL_HINSTANCE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWL_HINSTANCE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWL_HWNDPARENT>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWL_HWNDPARENT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWL_ID>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWL_ID;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWL_STYLE>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWL_STYLE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWL_USERDATA>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWL_USERDATA;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw#GWL_WNDPROC>
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::GWL_WNDPROC;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_3DDKSHADOW>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_3DDKSHADOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_3DFACE>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_3DFACE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_3DHIGHLIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_3DHIGHLIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_3DHILIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_3DHILIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_3DLIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_3DLIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_3DSHADOW>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_3DSHADOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_ACTIVEBORDER>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_ACTIVEBORDER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_ACTIVECAPTION>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_ACTIVECAPTION;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_APPWORKSPACE>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_APPWORKSPACE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_BACKGROUND>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_BACKGROUND;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_BTNFACE>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_BTNFACE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_BTNHIGHLIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_BTNHIGHLIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_BTNHILIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_BTNHILIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_BTNSHADOW>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_BTNSHADOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_BTNTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_BTNTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_CAPTIONTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_CAPTIONTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_DESKTOP>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_DESKTOP;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_GRADIENTACTIVECAPTION>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_GRADIENTACTIVECAPTION;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_GRADIENTINACTIVECAPTION>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_GRADIENTINACTIVECAPTION;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_GRAYTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_GRAYTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_HIGHLIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_HIGHLIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_HIGHLIGHTTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_HIGHLIGHTTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_HOTLIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_HOTLIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_INACTIVEBORDER>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_INACTIVEBORDER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_INACTIVECAPTION>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_INACTIVECAPTION;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_INACTIVECAPTIONTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_INACTIVECAPTIONTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_INFOBK>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_INFOBK;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_INFOTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_INFOTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_MENU>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_MENU;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_MENUBAR>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_MENUBAR;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_MENUHILIGHT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_MENUHILIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_MENUTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_MENUTEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_SCROLLBAR>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_SCROLLBAR;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_WINDOW>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_WINDOW;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_WINDOWFRAME>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_WINDOWFRAME;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#COLOR_WINDOWTEXT>\
pub use ::windows_sys::Win32::Graphics::Gdi::COLOR_WINDOWTEXT;

/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint#MONITOR_DEFAULTTONEAREST>\
pub use ::windows_sys::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint#MONITOR_DEFAULTTONULL>\
pub use ::windows_sys::Win32::Graphics::Gdi::MONITOR_DEFAULTTONULL;
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint#MONITOR_DEFAULTTOPRIMARY>\
pub use ::windows_sys::Win32::Graphics::Gdi::MONITOR_DEFAULTTOPRIMARY;

/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#parameters>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WHEEL_DELTA;

/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#MK_CONTROL>\
pub use ::windows_sys::Win32::System::SystemServices::MK_CONTROL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#MK_LBUTTON>\
pub use ::windows_sys::Win32::System::SystemServices::MK_LBUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#MK_MBUTTON>\
pub use ::windows_sys::Win32::System::SystemServices::MK_MBUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#MK_RBUTTON>\
pub use ::windows_sys::Win32::System::SystemServices::MK_RBUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#MK_SHIFT>\
pub use ::windows_sys::Win32::System::SystemServices::MK_SHIFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#MK_XBUTTON1>\
pub use ::windows_sys::Win32::System::SystemServices::MK_XBUTTON1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel#MK_XBUTTON2>\
pub use ::windows_sys::Win32::System::SystemServices::MK_XBUTTON2;

/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_0>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_0;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_2>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_2;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_3>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_3;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_4>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_4;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_5>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_5;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_6>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_6;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_7>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_7;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_8>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_8;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_9>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_9;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_A>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_A;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ACCEPT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ACCEPT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ADD>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ADD;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_APPS>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_APPS;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ATTN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ATTN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_B>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_B;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BACK>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BACK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BROWSER_BACK>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BROWSER_BACK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BROWSER_FAVORITES>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BROWSER_FAVORITES;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BROWSER_FORWARD>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BROWSER_FORWARD;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BROWSER_HOME>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BROWSER_HOME;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BROWSER_REFRESH>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BROWSER_REFRESH;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BROWSER_SEARCH>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BROWSER_SEARCH;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_BROWSER_STOP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_BROWSER_STOP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_C>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_C;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_CANCEL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CANCEL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_CAPITAL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CAPITAL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_CLEAR>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CLEAR;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_CONTROL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_CONVERT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CONVERT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_CRSEL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CRSEL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_D>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_D;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_DECIMAL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_DECIMAL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_DELETE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_DELETE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_DIVIDE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_DIVIDE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_DOWN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_DOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_E>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_E;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_END>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_END;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_EREOF>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_EREOF;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ESCAPE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_EXECUTE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_EXECUTE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_EXSEL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_EXSEL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F10>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F10;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F11>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F11;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F12>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F12;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F13>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F13;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F14>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F14;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F15>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F15;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F16>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F16;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F17>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F17;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F18>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F18;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F19>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F19;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F2>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F2;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F20>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F20;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F21>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F21;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F22>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F22;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F23>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F23;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F24>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F24;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F3>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F3;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F4>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F4;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F5>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F5;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F6>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F6;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F7>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F7;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F8>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F8;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_F9>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_F9;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_FINAL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_FINAL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_G>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_G;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_A>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_A;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_B>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_B;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_DPAD_DOWN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_DPAD_DOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_DPAD_LEFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_DPAD_LEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_DPAD_RIGHT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_DPAD_RIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_DPAD_UP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_DPAD_UP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_LEFT_SHOULDER>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_LEFT_SHOULDER;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_LEFT_THUMBSTICK_DOWN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_LEFT_THUMBSTICK_DOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_LEFT_THUMBSTICK_LEFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_LEFT_THUMBSTICK_LEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_LEFT_THUMBSTICK_UP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_LEFT_THUMBSTICK_UP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_LEFT_TRIGGER>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_LEFT_TRIGGER;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_MENU>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_MENU;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_RIGHT_SHOULDER>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_RIGHT_SHOULDER;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_RIGHT_THUMBSTICK_UP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_RIGHT_THUMBSTICK_UP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_RIGHT_TRIGGER>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_RIGHT_TRIGGER;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_VIEW>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_VIEW;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_X>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_X;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_GAMEPAD_Y>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_GAMEPAD_Y;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_H>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_H;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_HANGEUL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_HANGEUL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_HANGUL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_HANGUL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_HANJA>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_HANJA;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_HELP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_HELP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_HOME>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_HOME;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_I>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_I;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ICO_00>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ICO_00;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ICO_CLEAR>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ICO_CLEAR;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ICO_HELP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ICO_HELP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_IME_OFF>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_IME_OFF;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_IME_ON>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_IME_ON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_INSERT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_INSERT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_J>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_J;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_JUNJA>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_JUNJA;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_K>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_K;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_KANA>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_KANA;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_KANJI>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_KANJI;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_L>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_L;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LAUNCH_APP1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LAUNCH_APP1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LAUNCH_APP2>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LAUNCH_APP2;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LAUNCH_MAIL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LAUNCH_MAIL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LAUNCH_MEDIA_SELECT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LAUNCH_MEDIA_SELECT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LBUTTON>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LBUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LCONTROL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LCONTROL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LEFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LMENU>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LSHIFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LSHIFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_LWIN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LWIN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_M>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_M;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MBUTTON>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MBUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MEDIA_NEXT_TRACK>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MEDIA_NEXT_TRACK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MEDIA_PLAY_PAUSE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MEDIA_PLAY_PAUSE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MEDIA_PREV_TRACK>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MEDIA_PREV_TRACK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MEDIA_STOP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MEDIA_STOP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MENU>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MENU;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MODECHANGE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MODECHANGE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_MULTIPLY>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MULTIPLY;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_N>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_N;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_ACCEPT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_ACCEPT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_CANCEL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_CANCEL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_DOWN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_DOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_LEFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_LEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_MENU>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_MENU;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_RIGHT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_RIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_UP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_UP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NAVIGATION_VIEW>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NAVIGATION_VIEW;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NEXT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NEXT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NONAME>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NONAME;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NONCONVERT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NONCONVERT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMLOCK>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMLOCK;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD0>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD0;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD2>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD2;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD3>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD3;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD4>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD4;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD5>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD5;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD6>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD6;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD7>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD7;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD8>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD8;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_NUMPAD9>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD9;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_O>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_O;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_102>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_102;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_2>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_2;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_3>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_3;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_4>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_4;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_5>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_5;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_6>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_6;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_7>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_7;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_8>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_8;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_ATTN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_ATTN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_AUTO>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_AUTO;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_AX>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_AX;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_BACKTAB>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_BACKTAB;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_CLEAR>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_CLEAR;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_COMMA>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_COMMA;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_COPY>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_COPY;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_CUSEL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_CUSEL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_ENLW>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_ENLW;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_FINISH>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_FINISH;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_FJ_JISHO>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_FJ_JISHO;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_FJ_LOYA>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_FJ_LOYA;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_FJ_MASSHOU>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_FJ_MASSHOU;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_FJ_ROYA>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_FJ_ROYA;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_FJ_TOUROKU>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_FJ_TOUROKU;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_JUMP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_JUMP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_MINUS>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_MINUS;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_NEC_EQUAL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_NEC_EQUAL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_PA1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_PA1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_PA2>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_PA2;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_PA3>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_PA3;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_PERIOD>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_PERIOD;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_PLUS>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_PLUS;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_RESET>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_RESET;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_OEM_WSCTRL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_OEM_WSCTRL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_P>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_P;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_PA1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_PA1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_PACKET>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_PACKET;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_PAUSE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_PAUSE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_PLAY>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_PLAY;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_PRINT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_PRINT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_PRIOR>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_PRIOR;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_PROCESSKEY>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_PROCESSKEY;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_Q>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_Q;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_R>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_R;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_RBUTTON>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RBUTTON;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_RCONTROL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RCONTROL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_RETURN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RETURN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_RIGHT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_RMENU>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RMENU;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_RSHIFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RSHIFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_RWIN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RWIN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_S>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_S;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SCROLL>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SCROLL;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SELECT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SELECT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SEPARATOR>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SEPARATOR;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SHIFT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SHIFT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SLEEP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SLEEP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SNAPSHOT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SNAPSHOT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SPACE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_SUBTRACT>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SUBTRACT;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_T>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_T;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_TAB>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_TAB;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_U>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_U;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_UP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_UP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_V>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_V;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_VOLUME_DOWN>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_VOLUME_DOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_VOLUME_MUTE>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_VOLUME_MUTE;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_VOLUME_UP>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_VOLUME_UP;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_W>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_W;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_X>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_X;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_XBUTTON1>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_XBUTTON1;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_XBUTTON2>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_XBUTTON2;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_Y>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_Y;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_Z>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_Z;
/// <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes#VK_ZOOM>\
pub use ::windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ZOOM;

/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_A>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_A;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_B>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_B;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_BACK>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_BACK;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_DPAD_DOWN>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_DOWN;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_DPAD_LEFT>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_LEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_DPAD_RIGHT>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_RIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_DPAD_UP>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_DPAD_UP;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_LEFT_SHOULDER>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_SHOULDER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_LEFT_THUMB>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_THUMB;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_RIGHT_SHOULDER>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_SHOULDER;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_RIGHT_THUMB>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_THUMB;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_START>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_START;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_X>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_X;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#XINPUT_GAMEPAD_Y>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_Y;

/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#sThumbLX:~:text=XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#sThumbLX:~:text=XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#sThumbLX:~:text=XINPUT_GAMEPAD_TRIGGER_THRESHOLD>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XINPUT_GAMEPAD_TRIGGER_THRESHOLD;
/// <https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke#:~:text=the%20range%200%E2%80%93-,XUSER_MAX_COUNT,-%E2%88%92%201%2C%20or%20XUSER_INDEX_ANY>\
pub use ::windows_sys::Win32::UI::Input::XboxController::XUSER_MAX_COUNT;

/// <https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--1000-1299-#ERROR_DEVICE_NOT_CONNECTED>
pub use ::windows_sys::Win32::Foundation::ERROR_DEVICE_NOT_CONNECTED;
/// <https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-#ERROR_SUCCESS>
pub use ::windows_sys::Win32::Foundation::ERROR_SUCCESS;

/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timegetdevcaps#return-value>\
pub use ::windows_sys::Win32::Media::MMSYSERR_ERROR;
/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timegetdevcaps#return-value>\
pub use ::windows_sys::Win32::Media::MMSYSERR_NOERROR;
/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod#return-value>\
pub use ::windows_sys::Win32::Media::TIMERR_NOCANDO;
/// <https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod#return-value>\
pub use ::windows_sys::Win32::Media::TIMERR_NOERROR;

/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize#return-value>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WVR_ALIGNBOTTOM;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize#return-value>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WVR_ALIGNLEFT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize#return-value>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WVR_ALIGNRIGHT;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize#return-value>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WVR_ALIGNTOP;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize#return-value>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WVR_HREDRAW;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize#return-value>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WVR_VALIDRECTS;
/// <https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize#return-value>\
pub use ::windows_sys::Win32::UI::WindowsAndMessaging::WVR_VREDRAW;

// ================================================================================================================================ //
