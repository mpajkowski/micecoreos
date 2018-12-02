#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![feature(abi_x86_interrupt)]
#![feature(asm)]

pub mod vga;
pub mod gdt;
pub mod interrupts;
pub mod serial;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
