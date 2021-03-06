#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate bootloader;

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub fn _start() -> ! {
    let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

    slice[0] = b'H';
    slice[1] = 0x02;
    slice[2] = b'e';
    slice[3] = 0x02;
    slice[4] = b'l';
    slice[5] = 0x02;
    slice[6] = b'l';
    slice[7] = 0x02;
    slice[8] = b'o';
    slice[9] = 0x02;
    slice[10] = b' ';
    slice[11] = 0x02;
    slice[12] = b'R';
    slice[13] = 0x02;
    slice[14] = b'u';
    slice[15] = 0x02;
    slice[16] = b's';
    slice[17] = 0x02;
    slice[18] = b't';
    slice[19] = 0x02;
    slice[20] = b'!';
    slice[21] = 0x02;

    loop {}
}