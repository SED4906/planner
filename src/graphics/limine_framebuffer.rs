use limine::request::FramebufferRequest;
use crate::PtrMut;

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn gfx_init() {
    let framebuffer = &mut *super::framebuffer::FRAMEBUFFER.lock();
    let response = FRAMEBUFFER_REQUEST.get_response().unwrap().framebuffers().next().unwrap();
    framebuffer.base = PtrMut(response.addr());
    framebuffer.width = response.width() as usize;
    framebuffer.height = response.height() as usize;
    framebuffer.pitch = response.pitch() as usize;
}