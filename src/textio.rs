#[cfg(feature = "fbprint")]
mod fbprint;

use core::fmt;
use spin::Mutex;

pub struct Writer {}
static WRITER: Mutex<Writer> = Mutex::new(Writer {});

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(feature = "fbprint")]
        fbprint::draw_console_string(s, 0xFFFFFFFF);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::textio::_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)) };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)) };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_println {
    () => {};
    ($($t:tt)*) => {};
}
