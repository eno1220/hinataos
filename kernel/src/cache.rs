use core::arch::asm;
use core::arch::x86_64::_mm_clflush;
use x86;

use crate::println;
#[allow(unused_imports)]
use crate::serial_println;

const PAGE_SIZE: usize = 4096;

#[inline(always)]
unsafe fn flush(addr: *const u8) {
    _mm_clflush(addr);
}

#[inline(always)]
unsafe fn flush_buffer(buffer: *const u8) {
    for i in 0..2 {
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
unsafe fn guess_bit_once(seed: u8, buffer: *mut u8) -> u8 {
    flush_buffer(buffer);

    buffer.add((seed as usize) * PAGE_SIZE).write_volatile(1);

    // 本当は 256 だけど、まあ文字範囲的に 80 で十分（256だと配列が大きすぎてクラッシュする）
    (0..2)
        .min_by_key(|i| probe(buffer.add(i * PAGE_SIZE)))
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
    const TRY_COUNT: usize = 100;
    let mut hit_counts = [0; 2];

    for _ in 0..TRY_COUNT {
        hit_counts[guess_bit_once(seed, buffer) as usize] += 1;
    }

    hit_counts
        .iter()
        .enumerate()
        .max_by_key(|(i, &count)| {
            println!("{}: {}", i, count);
            count
        })
        .unwrap()
        .0 as u8
}

pub fn cache() {
    /*static SAMPLE: &'static str = "Hinata OS";
    let sample = SAMPLE.as_ptr();*/
    let sample = 0b10101010 as u8;
    let mut buffer = [0u8; PAGE_SIZE * 2];

    for i in 0..8 {
        unsafe {
            let result = guess_bit(((sample >> i) & 1), buffer.as_mut_ptr());
            println!("{}", result);
        }
    }
}
