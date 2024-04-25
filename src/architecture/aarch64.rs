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
            asm!("msr DAIFSet, #0b1111", "wfe");
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
        let mut mpidr: usize;
        asm!("mrs {}, mpidr_el1", out(reg) mpidr);
        mpidr &= 3;
        if mpidr != 0 {
            wait_forever();
        }

        core::slice::from_raw_parts_mut(__bss_start as *mut u8, __bss_size).fill(0);

        let paging = _end as *mut usize;
        for page in 0..16 {
            *paging.add(page) = (0x1000000000 * page)
                | (1<<0) // PT_BLOCK
                | if page > 0 {1<<2} else {0} // PT_DEV else PT_MEM
                | (1<<6) // PT_RW
                | (3<<8) // PT_ISH
                | (1<<10) // PT_AF
                ;
        }

        asm!("msr mair_el2, {}", in(reg) 0x4004400ffusize);

        let mut tcr: usize;
        asm!("mrs {}, tcr_el2", out(reg) tcr);
        //let mut hcr: usize;
        //asm!("mrs {}, hcr_el1", out(reg) hcr);

        //hcr &= !(1usize<<34);
        tcr = (tcr & !0xC000) | 0x4000; // TG0
        tcr = (tcr & !0x70000) | 0x20000; // PS
        tcr = (tcr & !0x1F) | 0x18; // T0SZ
        tcr = (tcr & !0x3000) | 0x2000; // SH0
        tcr &= !0x200000;

        asm!("msr tcr_el2, {}", in(reg) tcr);
        asm!("msr ttbr0_el2, {}", in(reg) (paging as usize + 1));
        //asm!("msr hcr_el1, {}", in(reg) hcr);
        let mut sctlr: usize;
        asm!("dsb ish", "isb" ,"mrs {}, sctlr_el2", out(reg) sctlr);
        sctlr |= 0xC00800;
        sctlr &= !((1<<25) |   // clear EE, little endian translation tables
            (1<<24) |   // clear E0E
            (1<<19) |   // clear WXN
            (1<<12) |   // clear I, no instruction cache
            (1<<4) |    // clear SA0
            (1<<3) |    // clear SA
            (1<<2) |    // clear C, no cache at all
            (1<<1));
        sctlr |= 1;
        asm!("msr sctlr_el2, {}", "isb", in(reg) sctlr);
    }
}
