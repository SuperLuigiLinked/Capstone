/*
 *  Crate: Wyn
 * Module: Examples - Utils - RwOpt
 */

//! Reader-Writer Lock around an Optional value.

// ================================================================================================================================ //

#![allow(unused)]

pub use std::sync::RwLock;

// ================================================================================================================================ //

#[repr(transparent)]
pub struct RwOpt<T>(pub RwLock<Option<T>>);

impl<T> RwOpt<T> {
    pub const fn new(opt: Option<T>) -> Self {
        Self(RwLock::new(opt))
    }

    pub fn write_opt<R>(&self, func: impl FnOnce(Option<&mut T>) -> R) -> R {
        let mut lock = self.0.write().unwrap();
        let opt = lock.as_mut();
        func(opt)
    }

    pub fn read_opt<R>(&self, func: impl FnOnce(Option<&T>) -> R) -> R {
        let lock = self.0.read().unwrap();
        let opt = lock.as_ref();
        func(opt)
    }

    pub fn write<R>(&self, func: impl FnOnce(&mut T) -> R) -> Option<R> {
        let mut lock = self.0.write().unwrap();
        let opt = lock.as_mut();
        opt.map(func)
    }

    pub fn read<R>(&self, func: impl FnOnce(&T) -> R) -> Option<R> {
        let lock = self.0.read().unwrap();
        let opt = lock.as_ref();
        opt.map(func)
    }

    pub fn take(&self) -> Option<T> {
        let mut lock = self.0.write().unwrap();
        lock.take()
    }

    pub fn set(&self, opt: Option<T>) {
        let mut lock = self.0.write().unwrap();
        *lock = opt;
    }
}

// ================================================================================================================================ //
