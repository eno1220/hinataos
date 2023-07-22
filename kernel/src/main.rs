// https://os.phil-opp.com/ja/minimal-rust-kernel/

#![no_std]
#![no_main]

use common::types::GraphicsInfo;
use core::arch::asm;
use core::panic::PanicInfo;
use x86_64::structures::port::PortWrite;

#[no_mangle]
pub extern "C" fn kernel_main(graphics_info: GraphicsInfo) -> ! {
    unsafe {
        u8::write_to_port(0x3f8, b'H');
        u8::write_to_port(0x3f8, b'e');
        u8::write_to_port(0x3f8, b'l');
        u8::write_to_port(0x3f8, b'l');
        u8::write_to_port(0x3f8, b'o');
        u8::write_to_port(0x3f8, b' ');
        u8::write_to_port(0x3f8, b'W');
        u8::write_to_port(0x3f8, b'o');
        u8::write_to_port(0x3f8, b'r');
        u8::write_to_port(0x3f8, b'l');
        u8::write_to_port(0x3f8, b'd');
        u8::write_to_port(0x3f8, b'!');
        u8::write_to_port(0x3f8, b'\n');
    }

    for i in 0..graphics_info.horizontal_resolution() {
        for j in 0..graphics_info.vertical_resolution() {
            unsafe {
                graphics_info
                    .frame_buffer_base()
                    .add((i * graphics_info.pixels_per_scan_line() + j) * 4)
                    .write_volatile(0xff);
            }
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
