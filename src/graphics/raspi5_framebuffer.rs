use crate::PtrMut;
use aligned::Aligned;
use aligned::A16;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

const PERIPHERAL_BASE: u64 = 0x107C000000;

const MBOX_TAG_SETPOWER: u32 = 0x28001;
const MBOX_TAG_SETCLKRATE: u32 = 0x38002;
const MBOX_TAG_SETPHYWH: u32 = 0x48003;
const MBOX_TAG_SETVIRTWH: u32 = 0x48004;
const MBOX_TAG_SETVIRTOFF: u32 = 0x48009;
const MBOX_TAG_SETDEPTH: u32 = 0x48005;
const MBOX_TAG_SETPXLORDR: u32 = 0x48006;
const MBOX_TAG_GETFB: u32 = 0x40001;
const MBOX_TAG_GETPITCH: u32 = 0x40008;
const MBOX_TAG_LAST: u32 = 0;

const MBOX_CH_PROP: u8 = 8;

static mut WIDTH: u32 = 0;
static mut HEIGHT: u32 = 0;
static mut PITCH: u32 = 0;
static mut IS_RGB: u32 = 0;

pub static mut MBOX: Aligned<A16, [u32; 36]> = Aligned([0; 36]);
const VIDEOCORE_MBOX: u64 = PERIPHERAL_BASE + 0x13880;
const MBOX_READ: u64 = VIDEOCORE_MBOX + 0;
const MBOX_POLL: u64 = VIDEOCORE_MBOX + 0x10;
const MBOX_SENDER: u64 = VIDEOCORE_MBOX + 0x14;
const MBOX_STATUS: u64 = VIDEOCORE_MBOX + 0x18;
const MBOX_CONFIG: u64 = VIDEOCORE_MBOX + 0x1C;
const MBOX_WRITE: u64 = VIDEOCORE_MBOX + 0x20;
const MBOX_RESPONSE: u32 = 0x80000000;
const MBOX_FULL: u32 = 0x80000000;
const MBOX_EMPTY: u32 = 0x40000000;
pub const MBOX_REQUEST: u32 = 0;

fn mmio_write(reg: u64, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u64) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

pub unsafe fn mbox_call(ch: u8) -> bool {
    let r: u32 = (unsafe { MBOX.as_ptr() as u32 } & !0xF) | ((ch & 0xF) as u32);
    while mmio_read(MBOX_STATUS) & MBOX_FULL != 0 {}
    mmio_write(MBOX_WRITE, r);

    loop {
        while mmio_read(MBOX_STATUS) & MBOX_EMPTY != 0 {}
        if r == mmio_read(MBOX_READ) {
            return unsafe { MBOX[1] } == MBOX_RESPONSE;
        }
    }
}

pub fn gfx_init() {
    let framebuffer = &mut *super::framebuffer::FRAMEBUFFER.lock();
    unsafe {
        MBOX[0] = 35 * 4;
        MBOX[1] = MBOX_REQUEST;

        MBOX[2] = MBOX_TAG_SETPHYWH;
        MBOX[3] = 8;
        MBOX[4] = 0;
        MBOX[5] = 1920;
        MBOX[6] = 1080;

        MBOX[7] = MBOX_TAG_SETVIRTWH;
        MBOX[8] = 8;
        MBOX[9] = 0;
        MBOX[10] = 1920;
        MBOX[11] = 1080;

        MBOX[12] = MBOX_TAG_SETVIRTOFF;
        MBOX[13] = 8;
        MBOX[14] = 0;
        MBOX[15] = 0;
        MBOX[16] = 0;

        MBOX[17] = MBOX_TAG_SETDEPTH;
        MBOX[18] = 4;
        MBOX[19] = 0;
        MBOX[20] = 32;

        MBOX[21] = MBOX_TAG_SETPXLORDR;
        MBOX[22] = 4;
        MBOX[23] = 0;
        MBOX[24] = 1;

        MBOX[25] = MBOX_TAG_GETFB;
        MBOX[26] = 8;
        MBOX[27] = 0;
        MBOX[28] = 4096;
        MBOX[29] = 0;

        MBOX[30] = MBOX_TAG_GETPITCH;
        MBOX[31] = 4;
        MBOX[32] = 0;
        MBOX[33] = 0;

        MBOX[34] = MBOX_TAG_LAST;

        if mbox_call(MBOX_CH_PROP) && MBOX[20] == 32 && MBOX[28] != 0 {
            MBOX[28] &= 0x3FFFFFFF;
            framebuffer.base = PtrMut(MBOX[28] as *mut u8);
            framebuffer.width = MBOX[10] as usize;
            framebuffer.height = MBOX[11] as usize;
            framebuffer.pitch = MBOX[33] as usize;
        }
    }
}
