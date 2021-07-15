#![no_std]
#![no_main]

#![feature(asm)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Scary World{}", "!");
    // panic!("Some panic message");
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
