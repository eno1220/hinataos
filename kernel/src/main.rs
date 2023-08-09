// https://os.phil-opp.com/ja/minimal-rust-kernel/

#![no_std]
#![no_main]

use common::types::{GraphicsInfo, MemoryMap};
use core::arch::asm;
use core::panic::PanicInfo;
use kernel::cache;
use kernel::console::Console;
use kernel::gdt;
use kernel::graphics::PixelInfo;
use kernel::interrupts;
use kernel::memory;
use kernel::paging;
use kernel::print::GLOBAL_POINTER;
use kernel::serial::{com_init, IO_ADDR_COM1};
use kernel::{println, serial_print};
use x86;
use x86_64::VirtAddr;
use x86_64::{
    registers::segmentation::*,
};
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

// 適当なメモリを確保してスタックにする
// スタックもユーザモードから使えるようにする

// メモリ全体をユーザが読み書きできるようにする
// スタックを切り替え（リターンアドレスをどうするかという工夫がある）
// 別の世界だとして考える or 戻ってくるようにする（戻り先のアドレスをスタックに積んでうまくずらしてやる）
// code segmentを書き換えると、ユーザからカーネルに戻れなくなってくる
// EFLAGSの中にあるIOPLを切り替えるとring3からでも出力できるようになる

#[no_mangle]
extern "C" fn kernel_main(graphics_info: &GraphicsInfo, memory_map: &MemoryMap) -> ! {
    interrupts::init();
    memory::init(memory_map);
    gdt::init();
    console_init(graphics_info);
    println!("Hello HinataOS{}", "!");
    paging::init();
    let (user_cs,user_ss) = gdt::get_user_segment();
    let app_stack = memory::alloc(0x1000);
    let new_rsp = app_stack + 0x1000 * 4096;
    unsafe{
        let time = x86::time::rdtsc();
        println!("{:08b}", time as u8);
        asm!(
            "mov rsp, {0}",
            "call {1}",
            "retfq",   
            in(reg) new_rsp,         
            in(reg) cache::cache as extern "C" fn(u8) -> (),
            in("dil") time as u8,
        );
    }

    /*
    unsafe {
        let time = x86::time::rdtsc();
        cache(time as u8);
        println!("{:08b}", time as u8);
        let time = x86::time::rdtsc();
        cache(time as u8);
        println!("{:08b}", time as u8);
        let time = x86::time::rdtsc();
        cache(time as u8);
        println!("{:08b}", time as u8);
    }*/
    loop {
        unsafe { asm!("hlt") };
    }
}

fn halt_loop() -> ! {
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
