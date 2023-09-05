// https://os.phil-opp.com/ja/minimal-rust-kernel/

#![no_std]
#![no_main]

use common::types::{GraphicsInfo, MemoryMap};
use core::arch::asm;
use core::panic::PanicInfo;
use kernel::gdt;
use kernel::interrupts;
use kernel::memory;
use kernel::paging;
use kernel::graphics;
use kernel::print::init_logger;
use kernel::serial::{serial_init, IO_ADDR_COM1};

#[no_mangle]
pub extern "C" fn kernel_entry(graphics_info: &GraphicsInfo, memory_map: &MemoryMap, new_rsp: u64) {
    unsafe {
        asm!(
            "mov rsp, {0}",
            "call kernel_main",
            in(reg) new_rsp,
            in("rdi") graphics_info,
            in("rsi") memory_map,
            clobber_abi("sysv64"),
        );
    }
}

#[no_mangle]
extern "C" fn kernel_main(graphics_info: &GraphicsInfo, memory_map: &MemoryMap) {
    console_init(graphics_info);
    gdt::init();
    interrupts::init();
    // todo:fix
    memory::init(memory_map);
    /*unsafe{
    MEMORY_MANAGER.lock().init(memory_map);
    }*/
    paging::init();
    log::info!("Hello HinataOS{}", "!");

    loop {
        unsafe { asm!("hlt") };
    }
}

#[no_mangle]
extern "C" fn halt_loop() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    loop {
        unsafe { asm!("hlt") };
    }
}

fn console_init(graphics_info: &GraphicsInfo) {
    // todo(eno1220): 名称変更
    serial_init(IO_ADDR_COM1);
    // todo(eno1220): 名称変更
    graphics::init(graphics_info);
    // todo(eno1220): 名称変更
    init_logger();
}
