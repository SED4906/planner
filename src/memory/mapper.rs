#[cfg(feature = "limine")]
#[path = "mapper/limine_memory_map.rs"]
pub mod limine_memory_map;
#[cfg(feature = "limine")]
pub use limine_memory_map as memmap;

#[cfg(feature = "raspi5")]
#[path = "mapper/raspi5_memory_map.rs"]
pub mod raspi5_memory_map;
#[cfg(feature = "raspi5")]
pub use raspi5_memory_map as memmap;
