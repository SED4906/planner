use limine::memory_map::EntryType;

use crate::memory::{freelist_dealloc, mm::PAGE_SIZE};

static LIMINE_MEMMAP_REQUEST: limine::request::MemoryMapRequest =
    limine::request::MemoryMapRequest::new();

pub fn memmap_init() {
    for entry in LIMINE_MEMMAP_REQUEST.get_response().unwrap().entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }
        let mut page = entry.base as usize;
        while page < (entry.base + entry.length) as usize {
            freelist_dealloc(page as *mut u8);
            page += PAGE_SIZE;
        }
    }
}
