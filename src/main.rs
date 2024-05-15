#![no_std]
#![no_main]

mod vga_buffer;

use core::{arch::asm, panic::PanicInfo};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";
static OS_42_MESSAGE: &[u8] = b"This is my first attempt at a kernel.";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
