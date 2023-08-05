// https://os.phil-opp.com/ja/minimal-rust-kernel/

#![no_std]
#![no_main]

use common::types::GraphicsInfo;
use core::arch::asm;
use core::panic::PanicInfo;
use kernel::console::Console;
use kernel::gdt;
use kernel::graphics::PixelInfo;
use kernel::interrupts;
use kernel::print::GLOBAL_POINTER;
use kernel::serial::{com_init, IO_ADDR_COM1};
use kernel::{println, serial_println};

#[repr(C, align(16))]
struct KernelStack([u8; 1024 * 1024]);

static mut KERNEL_STACK: KernelStack = KernelStack([0; 1024 * 1024]);

#[no_mangle]
pub extern "C" fn kernel_entry(graphics_info: &GraphicsInfo) {
    let new_rsp_addr = unsafe { KERNEL_STACK.0.as_ptr() as u64 + 1024 * 1024 };
    unsafe {
        asm!(
            "mov rsp, {0}",
            "call kernel_main",
            in(reg) new_rsp_addr,
            in("rdi") graphics_info,
            clobber_abi("sysv64"),
        );
    }
}

#[no_mangle]
extern "C" fn kernel_main(graphics_info: &GraphicsInfo) -> ! {
    gdt::init();
    interrupts::init_idt();
    console_init(graphics_info);
    println!("Hello HinataOS{}", "!");
    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {
        unsafe { asm!("hlt") };
    }
}

fn console_init(graphics_info: &GraphicsInfo) {
    com_init(IO_ADDR_COM1);
    let pixel_info = PixelInfo {
        buffer: graphics_info.frame_buffer_base() as *mut u8,
        width: graphics_info.horizontal_resolution(),
        height: graphics_info.vertical_resolution(),
        pixels_per_line: graphics_info.pixels_per_scan_line(),
    };
    let console = Console::new(
        pixel_info,
        graphics_info.horizontal_resolution(),
        graphics_info.vertical_resolution(),
        graphics_info.pixels_per_scan_line(),
    );
    GLOBAL_POINTER.set_console(console);
}
