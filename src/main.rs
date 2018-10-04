#![no_std]
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    writeln!(vga::WRITER.lock(), "{}.{}.{} Mice Core OS", 0, 0, 1).unwrap();
    writeln!(vga::WRITER.lock(), "").unwrap();

    for i in 0..10000 {
        write!(vga::WRITER.lock(), "{} ", i);
    }

    loop {}
}
