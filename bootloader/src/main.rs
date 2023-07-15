#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use uefi::prelude::*;

#[entry]
fn main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    uefi_services::println!("Hello, world!");

    // メモリマップ用のバッファを確保する
    let map_size = system_table.boot_services().memory_map_size().map_size + 4096;
    uefi_services::println!("map_size: {}", map_size);
    let mut map_buffer = Vec::with_capacity(map_size);
    // with_capacity だけだと、Vec の長さは 0 なので、
    // ここで明示的に長さを設定する
    unsafe {
        map_buffer.set_len(map_size);
    }
    // todo: Error handling
    let memory_map = system_table
        .boot_services()
        .memory_map(&mut map_buffer)
        .unwrap();

    memory_map.entries().for_each(|entry| {
        uefi_services::println!("entry: {:?}", entry);
    });

    system_table.boot_services().stall(100_000_000);
    Status::SUCCESS
}
