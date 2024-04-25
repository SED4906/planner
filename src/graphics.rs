#[cfg(feature = "framebuffer")]
pub mod framebuffer;
#[cfg(feature = "limine")]
pub mod limine_framebuffer;
#[cfg(feature = "limine")]
pub use limine_framebuffer as gfx;
#[cfg(feature = "raspi5")]
pub mod raspi5_framebuffer;
#[cfg(feature = "raspi5")]
pub use raspi5_framebuffer as gfx;