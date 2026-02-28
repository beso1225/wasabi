#![no_std]
#![no_main]

#[unsafe(no_mangle)]
fn efi_main(_image_hangle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let efi_graphics_output_protocol = locate_graphic_protocol(efi_system_table).unwrap();
    let vram_addr = efi_graphics_output_protocol.mode.frame_buffer_base;
    let vram_byte_size = efi_graphics_output_protocol.mode.frame_buffer_size;
    let vram = unsafe {
        slice::from_raw_parts_mut(vram_addr as *mut u32, vram_byte_size / size_of::<u32>())
    };
    // println!("Hello, world!");
    loop {}
}

use core::{panic::PanicInfo, slice};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}