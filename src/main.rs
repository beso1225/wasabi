#![no_std]
#![no_main]

#[unsafe(no_mangle)]
fn efi_main() {
    // println!("Hello, world!");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}