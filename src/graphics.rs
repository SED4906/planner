#[cfg(feature = "framebuffer")]
pub mod framebuffer;
#[cfg(feature = "limine_framebuffer")]
pub mod limine_framebuffer;
#[cfg(feature = "limine_framebuffer")]
pub use limine_framebuffer as gfx;