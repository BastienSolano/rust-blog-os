#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello world";

// We need to implement a function that will be called by rust upon panicking
// Note: this function should never return , so the return type is '!'
//       it is a diverging funciton
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Since we have no underlying operating system, we need to provide a starting point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // We need an unsafe block because rust cannot be sure that our raw pointer is correct
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // character byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color byte (0xb is light cyan)
        }
    }

    loop {}
}