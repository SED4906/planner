use core::ptr::null_mut;

use crate::PtrMut;
use spin::Mutex;

pub static FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer {
    base: PtrMut(null_mut()),
    width: 0,
    height: 0,
    pitch: 0,
});

#[derive(Default)]
pub struct Framebuffer {
    pub base: PtrMut<u8>,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

pub fn draw_pixel(x: usize, y: usize, color: u32) {
    let framebuffer = FRAMEBUFFER.lock();
    if framebuffer.base.0.is_null() {
        return;
    }
    unsafe {
        *(framebuffer.base.0.add(y * framebuffer.pitch + x * 4) as *mut u32) = color;
    }
}

static FONT: &[u8] = include_bytes!("FM-TOWNS.F08");
const FONT_HEIGHT: usize = 8;

pub fn draw_character(x: usize, y: usize, c: char, color: u32) {
    for py in y..y + FONT_HEIGHT {
        for px in x..x + 8 {
            if FONT[(c as u8) as usize * FONT_HEIGHT + py - y] & (128 >> (px - x)) != 0 {
                draw_pixel(px, py, color);
            } else {
                draw_pixel(px, py, !color);
            }
        }
    }
}