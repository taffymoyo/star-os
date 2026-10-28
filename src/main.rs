#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello from star-os!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The screen's text buffer lives at this memory address.
    let vga_buffer = 0xb8000 as *mut u8;

    // Go through each byte (character) of our message, with its index i.
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Each screen cell is 2 bytes: the character, then its colour.
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // light cyan
        }
    }

    loop {}
}