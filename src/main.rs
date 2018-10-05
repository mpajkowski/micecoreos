#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
extern crate std;

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;

#[macro_use]
mod vga;
#[macro_use]
mod serial;

extern crate x86_64;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    println!("{}.{}.{} Mice Core OS", 0, 0, 1);

    init_idt();

    for i in 0..=187 {
        print!("{}", i);
    }

    x86_64::instructions::int3();

    serial_println!("Hello, {}", ":)");

    loop {}
}
