use core::arch::asm;
use core::ptr::addr_of_mut;

pub fn wait_forever() -> ! {
    unsafe {
        loop {
            asm!("wfe");
        }
    }
}

pub fn hang_forever() -> ! {
    unsafe {
        loop {
            asm!("msr DAIFSet, 0b1111", "wfe");
        }
    }
}

extern "C" {
    static _end: usize;
    static __bss_start: usize;
    static __bss_size: usize;
}

pub fn cpu_init() {
    unsafe {
        let mut mpidr = 0usize;
        asm!("mrs {}, mpidr_el1", in(reg) mpidr);
        mpidr &= 3;
        if mpidr != 0 {
            wait_forever();
        }
        core::slice::from_raw_parts_mut(__bss_start as *mut u8, __bss_size).fill(0);

        asm!("msr mair_el2, {}", in(reg) 0x4004400ffusize);
        let paging = _end as *mut usize;
        for page in 0..64 {
            *paging.add(page) = 0x40000000000 * page
                | (1<<0) // PT_BLOCK
                | (1<<6) // PT_RW
                | (3<<8) // PT_ISH
                | (1<<10) // PT_AF
                ;
        }
        asm!("msr ttbr0_el2, {}", in(reg) paging);

        let mut mmfr0 = 0usize;
        asm!("mrs {}, id_aa64mmfr0_el1", out(reg) mmfr0);

        let mut tcr = 0usize;
        asm!("mrs {}, tcr_el2", out(reg) tcr);
        let mut hcr = 0usize;
        asm!("mrs {}, hcr_el2", out(reg) hcr);
        let mut sctlr = 0usize;
        asm!("mrs {}, sctlr_el2", out(reg) sctlr);

        hcr &= !(1usize<<34);
        tcr = (tcr & !0xC000) | 0x4000; // TG0
        tcr = (tcr & !0x70000)
            | ((mmfr0 & 0x7) << 16); // PS
        tcr = (tcr & !0x1F) | 0x10; // T0SZ
        tcr &= !0x200000;

        sctlr |= 1;
        sctlr &= !0x2000000;

        asm!("msr tcr_el2, {}", in(reg) tcr);
        asm!("msr hcr_el2, {}", in(reg) hcr);
        asm!("msr sctlr_el2, {}", in(reg) sctlr);
    }
}
