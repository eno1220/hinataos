// https://os.phil-opp.com/ja/minimal-rust-kernel/

#![no_std]
#![no_main]

use common::types::GraphicsInfo;
use core::panic::PanicInfo;
use kernel::graphics::graphics_init;
use kernel::serial_println;

#[no_mangle]
pub extern "C" fn kernel_main(graphics_info: GraphicsInfo) -> ! {
    serial_println!("Hello,world{}", "!");
    graphics_init(graphics_info);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{info}");
    loop {}
}
