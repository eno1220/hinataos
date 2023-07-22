// https://os.phil-opp.com/ja/minimal-rust-kernel/

#![no_std]
#![no_main]

use common::types::GraphicsInfo;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main(graphics_info: GraphicsInfo) -> ! {
    for i in 0..graphics_info.horizontal_resolution() {
        for j in 0..graphics_info.vertical_resolution() {
            unsafe {
                graphics_info
                    .frame_buffer_base()
                    .add((i * graphics_info.pixels_per_scan_line() + j )* 4)
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
