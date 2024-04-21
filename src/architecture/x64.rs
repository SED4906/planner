use core::arch::asm;

use spin::Mutex;
use x86_64::{instructions::tables::load_tss, registers::segmentation::{Segment, CS, SS}, structures::{gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector}, tss::TaskStateSegment}};

pub fn wait_forever() -> ! {
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

pub fn hang_forever() -> ! {
    unsafe {
        loop {
            asm!("cli;hlt");
        }
    }
}

static GDT: Mutex<GlobalDescriptorTable> = Mutex::new(GlobalDescriptorTable::empty());
static TSS: TaskStateSegment = TaskStateSegment::new();

pub fn cpu_init() {
    let mut gdt = GDT.lock();
    gdt.append(Descriptor::kernel_code_segment());
    gdt.append(Descriptor::kernel_data_segment());
    gdt.append(Descriptor::user_code_segment());
    gdt.append(Descriptor::user_data_segment());
    gdt.append(Descriptor::tss_segment(&TSS));
    unsafe{
        gdt.load_unsafe();
        CS::set_reg(SegmentSelector(0x08));
        SS::set_reg(SegmentSelector(0x10));
        load_tss(SegmentSelector(0x28));
    }
}