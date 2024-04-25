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

pub fn draw_rect(x0: usize, y0: usize, x1: usize, y1: usize, color: u32, fill: Option<u32>) {
    let mut y = y0;
    while y <= y1 {
        let mut x = x0;
        while x <= x1 {
            if (x == x0 || x == x1) || (y == y0 || y == y1) {
                draw_pixel(x, y, color);
            } else if let Some(fill) = fill {
                draw_pixel(x, y, fill);
            }
            x += 1;
        }
        y += 1;
    }
}

pub fn draw_line(x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    let dx = x1 as isize - x0 as isize;
    let dy = y1 as isize - y0 as isize;
    let mut x = x0;
    let mut y = y0;
    let mut p = 2 * dy as isize - dx as isize;
    while x < x1 {
        draw_pixel(x, y, color);
        if p >= 0 {
            y += 1;
            p = p + 2 * dy - 2 * dx;
        } else {
            p = p + 2 * dy;
        }
        x += 1;
    }
}

pub fn draw_circle(x0: usize, y0: usize, radius: usize, color: u32, fill: Option<u32>) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0isize;
    while x >= y {
        if let Some(fill) = fill {
            draw_line(x0 - y, y0 + x, x0 + y, y0 + x, fill);
            draw_line(x0 - x, y0 + y, x0 + x, y0 + y, fill);
            draw_line(x0 - x, y0 - y, x0 + x, y0 - y, fill);
            draw_line(x0 - y, y0 - x, x0 + y, y0 - x, fill);
        }
        draw_pixel(x0 - y, y0 + x, color);
        draw_pixel(x0 + y, y0 + x, color);
        draw_pixel(x0 - x, y0 + y, color);
        draw_pixel(x0 + x, y0 + y, color);
        draw_pixel(x0 - x, y0 - y, color);
        draw_pixel(x0 + x, y0 - y, color);
        draw_pixel(x0 - y, y0 - x, color);
        draw_pixel(x0 + y, y0 - x, color);

        if err <= 0 {
            y += 1;
            err += 2 * y as isize + 1;
        }

        if err > 0 {
            x -= 1;
            err -= 2 * x as isize + 1;
        }
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
