#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(pointer_is_aligned_to)]
#![no_std]
#![no_main]

pub mod architecture;
pub mod graphics;
pub mod memory;
pub mod textio;

use core::ptr::{null, null_mut};

pub struct Ptr<T>(pub *const T);
pub struct PtrMut<T>(pub *mut T);

unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}

unsafe impl<T> Send for PtrMut<T> {}
unsafe impl<T> Sync for PtrMut<T> {}

impl<T> Default for Ptr<T> {
    fn default() -> Self {
        Self(null())
    }
}

impl<T> Default for PtrMut<T> {
    fn default() -> Self {
        Self(null_mut())
    }
}
