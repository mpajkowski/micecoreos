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
    writeln!(vga::WRITER.lock(), "{}:{} Hello numbers", 14, 37).unwrap();
    panic!("Bye!");
}
