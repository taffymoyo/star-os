#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Welcome to star-os!";

fn write_cell(row: usize, col: usize, byte: u8) {

    // 0xb8000 = where the VGA colour text-mode buffer is mapped in memory (fixed by PC hardware)
    // as converts a value from one type to another
    let vga_buffer = 0xb8000 as *mut u8; // *mut u8 = raw pointer type, vga_buffer holds 0xb8000 as an address, at this address lives a u8
    let offset = (row * 80 + col) * 2; // writing the grid space
    unsafe { // tells the compiler to perform an unsafe action and not to complain
        vga_buffer.add(offset).write_volatile(byte); // makes a new pointer 0xb8000 + offset (add is an unsafe operation, moving a pointer is undefined behaviour)
        vga_buffer.add(offset + 1).write_volatile(0xe); // one byte after the char byte (VGA specific)  is the colour byte, 0xb is light cyan 
    }
}

fn write_string(row: usize, col: usize, s: &[u8]) {
    for (i, &byte) in s.iter().enumerate() { // iter returns references (the address of each char in s) so the for loop expects these references and unpacks them
        write_cell(row, i + col, byte); // byte holds the 
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    write_string(2, 31, HELLO);

    loop {}
}