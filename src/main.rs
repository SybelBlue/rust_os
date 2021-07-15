#![no_std]
#![no_main]

#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_string("Hello \n");
    vga_buffer::WRITER.lock().write_string("again");

    loop {}
}