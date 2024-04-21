use spin::Mutex;
use crate::graphics::framebuffer::{draw_character, FRAMEBUFFER};

static CHAR_X: Mutex<usize> = Mutex::new(0);
static CHAR_Y: Mutex<usize> = Mutex::new(0);

pub fn draw_console_string(s: &str, color: u32) {
    let x = &mut *CHAR_X.lock();
    let y = &mut *CHAR_Y.lock();
    for c in s.chars() {
        match c {
            '\r' => {
                *x = 0;
            }
            '\n' => {
                *x = 0;
                *y += 8;
            }
            _ => {
                draw_character(*x, *y, c, color);
                *x += 8;
            }
        }
        let framebuffer = FRAMEBUFFER.lock();
        if *x > framebuffer.width {
            *x = 0;
            *y += 8;
        }
        if *y > framebuffer.height {
            *x = 0;
            *y = 0;
        }
    }
}