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
