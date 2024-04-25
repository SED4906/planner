#[cfg(target_arch = "x86_64")]
pub mod x64;
use spin::Mutex;
#[cfg(target_arch = "x86_64")]
pub use x64 as mm;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64 as mm;

pub mod mapper;

use crate::PtrMut;
use core::ptr::null_mut;

struct Freelist(PtrMut<Freelist>);

static FREELIST: Mutex<Freelist> = Mutex::new(Freelist(PtrMut(null_mut())));

pub fn freelist_alloc() -> *mut u8 {
    let mut freelist = FREELIST.lock();
    let result = freelist.0 .0;
    if result.is_null() {
        return null_mut();
    }
    freelist.0 = unsafe { PtrMut((*result).0 .0) };
    result as *mut u8
}

pub fn freelist_dealloc(ptr: *mut u8) {
    let mut freelist = FREELIST.lock();
    if !ptr.is_aligned_to(mm::PAGE_SIZE) {
        return;
    }
    unsafe {
        (*(ptr as *mut *mut Freelist)) = freelist.0 .0;
    }
    freelist.0 = PtrMut(ptr as *mut Freelist);
}

pub struct PageFlags {
    pub present: bool,
    pub read_write: bool,
    pub user_writable: bool,
    pub accessed: bool,
    pub huge_page: bool,
    pub no_execute: bool,
}
