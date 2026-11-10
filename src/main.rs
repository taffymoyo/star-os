#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello from star-os!";

fn write_cell(row: usize, col: usize, byte: u8) {

    // 0xb8000 = where the VGA colour text-mode buffer is mapped in memory (fixed by PC hardware)
    // as converts a value from one type to another
    let vga_buffer = 0xb8000 as *mut u8; // *mut u8 = raw pointer type, vga_buffer holds 0xb8000 as an address, at this address lives a u8
    let offset = (row * 80 + col) * 2;
    unsafe {
        vga_buffer.add(offset).write_volatile(byte);
        vga_buffer.add(offset + 1).write_volatile(0xb);
    }
}

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