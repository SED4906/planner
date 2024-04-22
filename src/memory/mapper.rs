#[cfg(feature = "limine")]
#[path = "mapper/limine_memory_map.rs"]
pub mod limine_memory_map;
#[cfg(feature = "limine")]
pub use limine_memory_map as memmap;