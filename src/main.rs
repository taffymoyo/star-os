#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::read};
use core::fmt;
use core::fmt::Write;

struct Writer {
    row: usize,
    col: usize,
    colour: u8,
}

/// Writer struct responsible for writing characters
impl Writer { 

    /// Writes a new line
    /// If line is greater than 24 (out of VGA buffer), move existing lines up by one row and add a new blank row at line 24
    fn new_line(&mut self) {
        self.col = 0;
        self.row += 1;
        if self.row >= 25 {
            scroll();
            self.row = 24;
        }
    }

    /// Writes one byte at the cursor then advances the cursor
    fn write_byte(&mut self, byte: u8) { 
        if byte == b'\n' {
            self.new_line();
        } else {
            if self.col >= 80 {
                self.new_line();
            }
            write_cell(self.row, self.col, byte, self.colour);
            self.col += 1; // write the next char at the next position
        }
    }

    /// Writes a full string by repeatedly calling write byte method
    fn write_string(&mut self, s: &[u8]) {
        for &byte in s.iter() { // iter() returns addresses, &byte here follows the address to the value
            self.write_byte(byte); // byte holds the 
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s.as_bytes()); // write_string takes bytes, so s must be converted
        Ok(()) // return type expects an fmt::Result, which is an enum
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


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

fn scroll() {
    for row in 1..25 {
        for col in 0..80 {
            let (ch, colour) = read_cell(row, col);
            write_cell(row - 1, col, ch, colour);
        }
    }
    for col in 0..80 {
        write_cell(24, col, b' ', 0x07);
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

    clear_screen();
    let mut w = Writer { row: 0, col: 0, colour: 0x0a };
    write!(w, "1 + 1 = {}\n", 1 + 1).unwrap();
    write!(w, "hex of 255 is {:x}\n", 255).unwrap();
    loop {}
}
