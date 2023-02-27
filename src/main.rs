#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

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
    println!("Hello World{}", "!");

    loop {}
}