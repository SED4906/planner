#[cfg(target_arch = "x86_64")]
pub mod x64;
#[cfg(target_arch = "x86_64")]
pub use x64 as cpu;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64 as cpu;