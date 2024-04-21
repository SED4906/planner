#![no_std]
#![no_main]

use planner::architecture::cpu;
use planner::graphics::gfx;
use planner::println;

#[no_mangle]
fn _start() -> ! {
    cpu::cpu_init();
    gfx::gfx_init();
    println!("Hello, world!");
    cpu::wait_forever();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
	println!("{info}");
	cpu::hang_forever();
}
