#[cfg(feature = "framebuffer")]
pub mod framebuffer;
#[cfg(not(feature = "raspi5"))]
#[cfg(feature = "limine_framebuffer")]
pub mod limine_framebuffer;
#[cfg(not(feature = "raspi5"))]
#[cfg(feature = "limine_framebuffer")]
pub use limine_framebuffer as gfx;
#[cfg(feature = "raspi5")]
pub mod videocore_framebuffer;
#[cfg(feature = "raspi5")]
pub use videocore_framebuffer as gfx;