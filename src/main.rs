#![no_std]
#![no_main]

use planner::architecture::cpu;
use planner::graphics::{framebuffer, gfx};
use planner::memory::mm::{kernel_pagemap, map_to};
use planner::memory::{freelist_alloc, mm, PageFlags};
use planner::{print, println};

#[no_mangle]
fn _start() -> ! {
    cpu::cpu_init();
    gfx::gfx_init();
    mm::memory_init();
    println!("Hello, world!");
    cpu::wait_forever();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    cpu::hang_forever();
}
