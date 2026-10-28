# My Notes 

## Tools

QEMU - emulator that boots my OS on a fake computer

## Definitions

![no_std], so rust doesn't link the standard library which relies on the OS to do, cant use a std lib

![no_main]
Usually rust programs start with a hidden runtime that calls main, requiers an OS too

#[panic_handler] 
std lib usually prints a message and exits on eror, we need to define it ourselves, loop {} freezes the CPU for now.

#[no_mangle] 
rust usually scrambles names internally, this keeps names like _start the same so bootloader can find it.

[build] target
target computer that rust compiles for, otherwise it defaults to the regular computer OS (in my case MacOS)

runner 
says how to start the program since a kernal cant be run directly, so bootimage runner wraps it with the booatloader into a disk image, and QEMU is launcehd with it.

fn _start() -> !
- bootloader goes here when CPU is ready. 
- the ! is the never type reserved for functinos that done finish.

### Operating System

- manages device hardware and software resources
    - why? So the computer can function 

- bridges users, applications and hardware 
    - why? Hardware and software communicate in differnet languages, the OS translates

### Kernel

- program at the heart of an OS
- controls hardware and software components
    - why? Apps cant access hardware directly, they ask the kernel

- has FULL control of the hardware
    - why? For security and predictable behaviour

### Calling Convention
- Tells the function to follow the C "ABI" (application binary interface)
- extern "C"
- ABI = details code has to agree upon to work together, C is a common ABI.

- Currently theres two different compiled programs
    1. Kernel
    2. Bootloader
They both must agree on where arguments go, how to return arguments and other small operations. Bootloader only jumps to the _start() function and from there operations are handled by the rust source code.

### Bootloader
- Loaded after firmware (BIOS) has alerady run.
- Inherits an old CPU in 16 bit real mode.
- Converts cpu from 16-32-64 bit mode, sets up stack, copy the kernal from disk to memory etc... its final act is to jump to the start function.