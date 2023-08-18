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
    memory::init(memory_map);
    paging::init();
    log::info!("Hello HinataOS{}", "!");

    /*
    // 2GBの先頭pointer
    let p: *mut u8 = 0x80000000 as *mut u8;
    unsafe {
        *p = 10;
    }

    let app_stack = memory::alloc(0x1000);
    let new_rsp = app_stack + 0x1000 * 4096 - 64;
    let (user_code_segment, user_data_segment) = gdt::get_user_segment();
    unsafe {
        // p218 vol3 sdm error codeはpopしておく
        // RFLAGSは今のやつでOK(IOPLは3にしておく→ユーザモードでもIO空間にアクセスできるようになる)
        // iretじゃないといけない（どうじにssとcsを同時に切り替える、stackにつむ）
        let time = x86::time::rdtsc();
        Cr4::update(|cr4| {
            /*cr4.insert(Cr4Flags::TIMESTAMP_DISABLE);*/
            cr4.insert(Cr4Flags::PERFORMANCE_MONITOR_COUNTER);
        });
        asm!(
            "push {0}",
            "push {1}",
            "mov eax, 0x3016", //todo: シリアル出力時
            "push rax",
            "push {2}",
            "push {3}",
            "iretq",
            in(reg) user_data_segment as u64,
            in(reg) new_rsp,
            in(reg) user_code_segment as u64,
            in(reg) cache::cache as extern "C" fn(u8) -> (),
            in("dil") time as u8,
        )
    };
    */
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
