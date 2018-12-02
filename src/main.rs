#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![allow(dead_code)]

use core::panic::PanicInfo;

use micecoreos::{print, println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use micecoreos::interrupts::PICS;

    micecoreos::gdt::init();
    micecoreos::interrupts::init_idt();

    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("{}.{}.{} Mice Core OS", 0, 0, 1);
    micecoreos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    micecoreos::hlt_loop();
}
