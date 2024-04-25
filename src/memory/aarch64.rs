use core::arch::asm;
use crate::memory::PageFlags;
use crate::memory::mapper::memmap;

pub const PAGE_SIZE: usize = 16384;
pub const PAGE_MAX_LEVELS: usize = 3;

pub fn kernel_pagemap() -> usize {
    let mut paging;
    unsafe {asm!("mrs {}, ttbr0_el2", out(reg) paging);}
    paging
}

pub fn map_to(pagemap: usize, vpage: usize, pframe: usize, flags: PageFlags) -> Result<(), usize> {
    Ok(())
}

pub fn page_flags(pf: PageFlags) -> usize {
    let mut flags = 0;
    if pf.present {flags |= 1;}
    if !pf.read_write {flags |= 1<<7;}
    if pf.user_writable {flags |= 1<<6;}
    if pf.accessed {flags |= 1<<10;}
    if !pf.huge_page {flags |= 1<<1;}
    if pf.no_execute {flags |= 3<<53;}
    flags
}

pub fn memory_init() {
    memmap::memmap_init();
}