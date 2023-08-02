use core::arch::asm;
use core::arch::x86_64::_mm_clflush;
use x86;

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
unsafe fn guess_bit_once(seed: *const u8, buffer: *mut u8) -> u8 {
    flush_buffer(buffer);

    buffer
        .add(seed as usize * PAGE_SIZE)
        .write_volatile(1);

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
unsafe fn guess_bit(seed: *const u8, buffer: *mut u8) -> u8 {
    const TRY_COUNT: usize = 100;
    let mut hit_counts = [0; 128];

    for _ in 0..TRY_COUNT {
        hit_counts[guess_bit_once(seed, buffer) as usize] += 1;
    }

    hit_counts
        .iter()
        .enumerate()
        .max_by_key(|(i, &count)| {
            //serial_println!("{}: {}", i, count);
            count
        })
        .unwrap()
        .0 as u8
}

pub fn cache() {
    let sample: u8 = 0b10101010;
    // 2ページ分のメモリを確保
    // これヒープでやらないとダメなのかな？
    let mut buffer = [0u8; PAGE_SIZE * 2];
    //let mut result: u8 = 0;

    let mut sum_no_cache_time = 0;
    // キャッシュ差の実験
    // キャッシュから払い出しているとき
    for i in 0..256 {
        unsafe {
            flush(buffer.as_ptr());
        }
        let time = calc_access_time(
            #[inline(always)]
            || {
                unsafe { buffer.as_ptr().read_volatile() };
            },
        );
        if i > 10 {
            sum_no_cache_time += time;
        }
    }

    serial_println!("sum_no_cache_time: {}", sum_no_cache_time);
    serial_println!("average_no_cache_time: {}", sum_no_cache_time / 256);

    let mut sum_cache_time = 0;
    unsafe { buffer.as_ptr().read_volatile() };
    // キャッシュが効いているとき
    for i in 0..256 {
        let time = calc_access_time(
            #[inline(always)]
            || {
                unsafe { buffer.as_ptr().read_volatile() };
            },
        );
        if i > 10 {
            sum_cache_time += time;
        }
    }

    serial_println!("sum_cache_time: {}", sum_cache_time);
    serial_println!("average_cache_time: {}", sum_cache_time / 256);
}
