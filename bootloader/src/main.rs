#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use elf::{endian::AnyEndian, ElfBytes};
use uefi::{prelude::*, table::boot};

#[entry]
fn main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

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

    // todo: 内容ごとに表示する
    memory_map.entries().for_each(|entry| {
        uefi_services::println!("entry: {:?}", entry);
    });

    let mut file_protocol = match boot_services.get_image_file_system(_image) {
        Ok(file_protocol) => file_protocol,
        Err(error) => {
            panic!("Failed to get image file system: {:?}", error);
        }
    };

    let root_dir = match file_protocol.read_dir(cstr16!(".")) {
        Ok(root_dir) => root_dir,
        Err(error) => {
            panic!("Failed to open root directory: {:?}", error);
        }
    };

    let kernel_file_info = root_dir
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if entry.file_name() == cstr16!("kernel.elf") {
                    Some(entry)
                } else {
                    None
                }
            }
            Err(error) => {
                panic!("Failed to read entry: {:?}", error);
            }
        })
        .next()
        .unwrap();

    uefi_services::println!("kernel_file_info: {:?}", kernel_file_info);

    let kernel_file = match file_protocol.read(cstr16!("kernel.elf")) {
        Ok(kernel_file) => kernel_file,
        Err(error) => {
            panic!("Failed to open kernel file: {:?}", error);
        }
    };

    let file = match ElfBytes::<AnyEndian>::minimal_parse(&kernel_file) {
        Ok(file) => {
            for section in file.section_headers().unwrap() {
                uefi_services::println!("section: {:?}", section);
            }
            file
        }
        Err(error) => {
            panic!("Failed to parse kernel file: {:?}", error);
        }
    };
    let entry_point = file.ehdr.e_entry;

    let mut first_addr = u64::MAX;
    let mut last_addr = u64::MIN;
    for program_header in file.segments().unwrap() {
        uefi_services::println!("program_header: {:?}", program_header);
        if program_header.p_type == elf::abi::PT_LOAD {
            let start_addr = program_header.p_vaddr;
            let end_addr = start_addr + program_header.p_memsz;
            if start_addr < first_addr {
                first_addr = start_addr;
            }
            if end_addr > last_addr {
                last_addr = end_addr;
            }
        }
    }

    uefi_services::println!("first_addr: {:x}", first_addr);
    uefi_services::println!("last_addr: {:x}", last_addr);

    let pages = ((last_addr - first_addr) + 0xfff) / 0x1000;

    let physical_addr = match boot_services.allocate_pages(
        uefi::table::boot::AllocateType::Address(first_addr),
        uefi::table::boot::MemoryType::LOADER_DATA,
        pages as usize,
    ) {
        Ok(physical_addr) => physical_addr,
        Err(error) => {
            panic!("Failed to allocate pages: {:?}", error);
        }
    };

    uefi_services::println!("physical_addr: {:x}", physical_addr);

    for program_header in file.segments().unwrap() {
        if program_header.p_type == elf::abi::PT_LOAD {
            let segment_addr = kernel_file.as_ptr() as u64 + program_header.p_offset;
            let segment_size = program_header.p_filesz;
            let copy_to = program_header.p_vaddr;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    segment_addr as *const u8,
                    copy_to as *mut u8,
                    segment_size as usize,
                );
            }
            let zero_size = program_header.p_memsz - program_header.p_filesz;
            unsafe {
                core::ptr::write_bytes(
                    (segment_size + segment_addr) as *mut u8,
                    0,
                    zero_size as usize,
                );
            }
        }
    }

    drop(file_protocol);

    let (_, _) = system_table.exit_boot_services();

    unsafe {
        let entry_point: extern "C" fn() -> ! = core::mem::transmute(entry_point);
        entry_point();
    }

    #[allow(unreachable_code)]
    Status::SUCCESS
}
