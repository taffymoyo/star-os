#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Welcome to star-os!";
static mut ROW: usize = 0;
static mut COL: usize = 0;

fn write_cell(row: usize, col: usize, byte: u8, colour: u8) {

    // 0xb8000 = where the VGA colour text-mode buffer is mapped in memory (fixed by PC hardware)
    // as converts a value from one type to another
    let vga_buffer = 0xb8000 as *mut u8; // *mut u8 = raw pointer type, vga_buffer holds 0xb8000 as an address, at this address lives a u8
    let offset = (row * 80 + col) * 2; // writing the grid space
    unsafe { // tells the compiler to perform an unsafe action and not to complain
        vga_buffer.add(offset).write_volatile(byte); // makes a new pointer 0xb8000 + offset (add is an unsafe operation, moving a pointer is undefined behaviour)
        vga_buffer.add(offset + 1).write_volatile(colour); // one byte after the char byte (VGA specific)  is the colour byte, 0xb is light cyan 
    }
}

fn read_cell(row: usize, col: usize) -> (u8, u8) {

    let vga_buffer = 0xb8000 as *mut u8; // *mut u8 = raw pointer type, vga_buffer holds 0xb8000 as an address, at this address lives a u8
    let offset = (row * 80 + col) * 2; 
    unsafe { 
        (
            vga_buffer.add(offset).read_volatile(), // makes a new pointer 0xb8000 + offset (add is an unsafe operation, moving a pointer is undefined behaviour), fetches the content of the character byte
            vga_buffer.add(offset + 1).read_volatile() // one byte after the char byte (VGA specific)  is the colour byte, doesn't write this time, fetches the contents of the colour byte
        ) // note we dont put a ; at the end of this tuple, because we want to return it.
    }
}

/// function for writing characters at the cursor position
fn write_byte(byte: u8, colour: u8) { 
    unsafe { // needs unsafe because ROW and COL are global variables and can be modified by anything
        if byte == b'\n' {
            COL = 0;
            ROW += 1;
        } else {
            if COL >= 80 {
                COL = 0;
                ROW += 1;
            }
            write_cell(ROW, COL, byte, colour);
            COL += 1; // write the next char at the next position
        }
    }
}

fn write_string(s: &[u8], colour: u8) {
    for &byte in s.iter() { // iter returns references (the address of each char in s) so the for loop expects these references and unpacks them
        write_byte(byte, colour); // byte holds the 
    }
}

fn clear_screen() {
    for row in 0..25 {
        for col in 0..80 {
            write_cell(row, col,b' ', 0x00); // rewrites all text with spaces which show as black
        }
    }
}

fn delay() { // debugging tool, acts as my async for now
    for _ in 0..1_000_000 {
        core::hint::spin_loop(); // tells the cpu to do useless instructions to delay time
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    write_cell(5, 5, b'A', 0x0e);
    let (ch, colour) = read_cell(5, 5);
    if ch == b'A' && colour == 0x0e {
        write_string(b"read_cell works", 0x0a);
    } else {
        write_string(b"read_cell BROKEN", 0x04);
    }
    loop {}
}   