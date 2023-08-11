// https://os.phil-opp.com/ja/minimal-rust-kernel/

#![no_std]
#![no_main]

use common::types::{GraphicsInfo, MemoryMap};
use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use kernel::{cache, print};
use kernel::console::Console;
use kernel::gdt;
use kernel::graphics::PixelInfo;
use kernel::interrupts;
use kernel::memory;
use kernel::paging;
use kernel::print::GLOBAL_POINTER;
use kernel::serial::{com_init, IO_ADDR_COM1};
use kernel::serial_println;
use kernel::{println, serial_print};
use x86;
use x86::vmx::vmcs::guest::CR4;
use x86_64::registers::control::{Cr4, Cr4Flags};
use x86_64::registers::segmentation::*;

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
extern "C" fn kernel_main(graphics_info: &GraphicsInfo, memory_map: &MemoryMap) {
    console_init(graphics_info);
    gdt::init();
    interrupts::init();
    memory::init(memory_map);
    paging::init();
    println!("Hello HinataOS{}", "!");

    // 2GBの先頭pointer
    let p: *mut u8 = 0x80000000 as *mut u8;
    unsafe {
        *p = 10;
    }

    let app_stack = memory::alloc(0x1000);
    let new_rsp = app_stack + 0x1000 * 4096 - 64;
    let (user_code_segment, user_data_segment) = gdt::get_user_segment();
    //println!("user_code_segment: {:x}", user_code_segment);
    //println!("user_data_segment: {:x}", user_data_segment);
    //println!("new_rsp: {:x}", new_rsp);
    //gdt::set_user_segment();
    unsafe {
        /*let new_ss = SegmentSelector(4<<3);
        SS::set_reg(new_ss);
        let new_ss = SegmentSelector(4<<3 | 3);
        DS::set_reg(new_ss);
        ES::set_reg(new_ss);
        FS::set_reg(new_ss);
        GS::set_reg(new_ss);

        let new_cs = SegmentSelector(3<<3 | 3);

        CS::set_reg(new_cs);

        // 落ちる原因
        let time = x86::time::rdtsc();
        println!("{:08b}", time as u8);
        cache::cache(time as u8);*/
        /*asm!(
            "call {0}",
            in(reg) cache::cache as extern "C" fn(u8) -> (),
            in("dil") time as u8,
        );*/
        // p218 vol3 sdm error codeはpopしておく
        // RFLAGSは今のやつでOK(IOPLは3にしておく→ユーザモードでもIO空間にアクセスできるようになる)
        // iretじゃないといけない（どうじにssとcsを同時に切り替える、stackにつむ）
        let time = x86::time::rdtsc();
        /*asm!(
            "push {0}",
            "push {1}",
            "push {2}",
            "push {3}",
            "iretq",
            in(reg) user_data_segment as u64,
            in(reg) new_rsp,
            in(reg) user_code_segment as u64,
            in(reg) halt_loop as extern "C" fn() -> !,
        );*/
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

#[no_mangle]
extern "C" fn halt_loop() -> ! {
    loop {}
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
