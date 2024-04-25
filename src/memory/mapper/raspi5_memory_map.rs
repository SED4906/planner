use crate::memory::aarch64::PAGE_SIZE;
use crate::memory::freelist_dealloc;

struct MemmapEntry {
    pub base: usize,
    pub length: usize,
}

static USABLE_MEMORY: &[MemmapEntry] = &[MemmapEntry {
    base: 0,
    length: 8 * 1024 * 1024 * 1024,
}];

pub fn memmap_init() {
    for entry in USABLE_MEMORY {
        let mut page = entry.base;
        while page < entry.base + entry.length {
            freelist_dealloc(page as *mut u8);
            page += PAGE_SIZE;
        }
    }
}
