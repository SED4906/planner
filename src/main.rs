#![no_std]
#![no_main]

use planner::architecture::cpu;
use planner::graphics::gfx;
use planner::memory::{freelist_alloc, mm};
use planner::{print, println};

#[no_mangle]
fn _start() -> ! {
    cpu::cpu_init();
    gfx::gfx_init();
    mm::memory_init();
    println!("Hello, world!");
    loop {
        let page = freelist_alloc();
        if page.is_null() {break;}
        print!("{} ", page as usize);
    }
    cpu::wait_forever();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
	println!("{info}");
	cpu::hang_forever();
}
