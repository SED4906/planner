pub const PAGE_SIZE: usize = 4096;
const PAGE_MAX_LEVELS: usize = 4;

#[path = "mapper.rs"]
pub mod mapper;

pub fn map_to(vpage: usize, pframe: usize, flags: usize) {
    
}

pub fn memory_init() {
    mapper::memmap::memmap_init();
}