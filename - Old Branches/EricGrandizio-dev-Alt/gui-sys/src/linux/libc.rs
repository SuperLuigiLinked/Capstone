/*
 *  Crate: GUI-Sys
 * Module: Linux - Libc
 */

//! Linux Libc bindings.
//!
//! # Dependencies
//! * <https://crates.io/crates/libc>
//!
//! # Documentation
//! * <https://en.cppreference.com/w/c>
//! * <https://www.kernel.org/doc/man-pages/>

// -------------------------------------------------------------------------------------------------------------------------------- //

use crate::common::c_types::*;

// ================================================================================================================================ //
// Macros
// -------------------------------------------------------------------------------------------------------------------------------- //

// ================================================================================================================================ //
// Types
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://man7.org/linux/man-pages/man2/epoll_wait.2.html#DESCRIPTION>
pub use ::libc::epoll_event as libc_epoll_event;

/// <https://man7.org/linux/man-pages/man2/epoll_wait.2.html#DESCRIPTION>
#[repr(C)]
#[cfg_attr(
    any(
        all(
            target_arch = "x86",
            not(target_env = "musl"),
            not(target_os = "android")
        ),
        target_arch = "x86_64"
    ),
    repr(packed)
)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data_t,
}

impl epoll_event {
    #[inline]
    pub const fn as_libc(&self) -> *const libc_epoll_event {
        self as *const _ as *const _
    }

    #[inline]
    pub fn as_libc_mut(&mut self) -> *mut libc_epoll_event {
        self as *mut _ as *mut _
    }
}

/// <https://man7.org/linux/man-pages/man2/epoll_wait.2.html#DESCRIPTION>
#[repr(C)]
pub union epoll_data_t {
    pub ptr: *mut ::core::ffi::c_void,
    pub fd: ::core::ffi::c_int,
    pub u32: u32,
    pub u64: u64,
}

// ================================================================================================================================ //
// Functions
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://man7.org/linux/man-pages/man2/read.2.html>
pub use ::libc::read;

/// <https://man7.org/linux/man-pages/man2/write.2.html>
pub use ::libc::write;

/// <https://man7.org/linux/man-pages/man2/open.2.html>
pub use ::libc::open;

/// <https://man7.org/linux/man-pages/man2/close.2.html>
pub use ::libc::close;

/// <https://man7.org/linux/man-pages/man2/eventfd.2.html>
pub use ::libc::eventfd;

/// <https://man7.org/linux/man-pages/man2/epoll_create1.2.html>
pub use ::libc::epoll_create1;

/// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html>
pub use ::libc::epoll_ctl;

/// <https://man7.org/linux/man-pages/man2/epoll_wait.2.html>
pub use ::libc::epoll_wait;

// ================================================================================================================================ //
// Constants
// -------------------------------------------------------------------------------------------------------------------------------- //

/// <https://man7.org/linux/man-pages/man2/eventfd.2.html#DESCRIPTION>
pub use ::libc::EFD_CLOEXEC;
/// <https://man7.org/linux/man-pages/man2/eventfd.2.html#DESCRIPTION>
pub use ::libc::EFD_NONBLOCK;
/// <https://man7.org/linux/man-pages/man2/eventfd.2.html#DESCRIPTION>
pub use ::libc::EFD_SEMAPHORE;

/// <https://man7.org/linux/man-pages/man2/epoll_create.2.html#DESCRIPTION>
pub use ::libc::EPOLL_CLOEXEC;

/// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html#DESCRIPTION>
pub use ::libc::EPOLLERR;
/// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html#DESCRIPTION>
pub use ::libc::EPOLLIN;
/// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html#DESCRIPTION>
pub use ::libc::EPOLLOUT;

/// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html#DESCRIPTION>
pub use ::libc::EPOLL_CTL_ADD;
/// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html#DESCRIPTION>
pub use ::libc::EPOLL_CTL_DEL;
/// <https://man7.org/linux/man-pages/man2/epoll_ctl.2.html#DESCRIPTION>
pub use ::libc::EPOLL_CTL_MOD;

// ================================================================================================================================ //
