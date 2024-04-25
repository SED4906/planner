use x86_64::registers::control::Cr3;

use crate::memory::mapper::memmap;
use super::{freelist_alloc, PageFlags};

pub const PAGE_SIZE: usize = 4096;
//const PAGE_MAX_LEVELS: usize = 4;

pub fn kernel_pagemap() -> usize {
    Cr3::read().0.start_address().as_u64() as usize
}

pub fn map_to(pagemap: usize, vpage: usize, pframe: usize, flags: PageFlags) -> Result<(), usize> {
    if flags.huge_page && (vpage & 0x1FFFFF != 0 || pframe & 0x1FFFFF != 0) {
        return Err(vpage);
    } else if vpage & 0xFFF != 0 || pframe & 0xFFF != 0 {
        return Err(vpage);
    }

    let entry_level4 = (vpage >> 39) & 0x1FF;
    let entry_level3 = (vpage >> 30) & 0x1FF;
    let entry_level2 = (vpage >> 21) & 0x1FF;
    let entry_level1 = (vpage >> 12) & 0x1FF;

    let pml3 = map_to_step(pagemap, entry_level4)?;
    let pml2 = map_to_step(pml3 & !0xFFF, entry_level3)?;
    if flags.huge_page {
        unsafe {*(pml2 as *mut usize).add(entry_level2) = pframe | page_flags(flags) | if entry_level4 >= 256 {0xFFFF<<48} else {0}; }
        return Ok(())
    }
    let pml1 = map_to_step(pml2 & !0xFFF, entry_level2)?;
    unsafe {*(pml1 as *mut usize).add(entry_level1) = pframe | page_flags(flags); }

    Ok(())
}

fn map_to_step(table: usize, index: usize) -> Result<usize, usize> {
    let entry = unsafe {*(table as *mut usize).add(index)};
    if entry & 1 == 0 {
        let page = freelist_alloc();
        if page.is_null() {
            return Err(entry);
        }
        unsafe {
            (*(page as *mut [u8;PAGE_SIZE])).fill(0);
            *(table as *mut usize).add(index) = page as usize | 7;
        }
        return Ok(page as usize | 7);
    }
    Ok(entry)
}

pub fn page_flags(pf: PageFlags) -> usize {
    let mut flags = 0;
    if pf.present {flags |= 1;}
    if pf.read_write {flags |= 1<<1;}
    if pf.user_writable {flags |= 1<<2;}
    if pf.accessed {flags |= 1<<5;}
    if pf.huge_page {flags |= 1<<7;}
    if pf.no_execute {flags |= 1<<63;}
    flags
}

pub fn memory_init() {
    memmap::memmap_init();
}