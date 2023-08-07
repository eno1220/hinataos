#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use common::types::{GraphicsInfo, PixelFormat, MemoryMap as CommonMemoryMap};
use elf::{endian::AnyEndian, ElfBytes};
use uefi::table::boot::{MemoryMap, MemoryDescriptor};
use uefi::{prelude::*, proto::console::gop::GraphicsOutput, table::boot::SearchType};

#[entry]
fn main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

    uefi_services::println!("Hello, world!");

    // メモリマップ用のバッファを確保する
    let map_size = system_table.boot_services().memory_map_size().map_size + 4096;
    uefi_services::println!("map_size: {}", map_size);
    let mut map_buffer = vec![0; map_size];
    let memory_map = system_table
        .boot_services()
        .memory_map(&mut map_buffer)
        .unwrap();

    pretty_print_memory_map(&memory_map);

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
            /*for section in file.section_headers().unwrap() {
                uefi_services::println!("section: {:?}", section);
            }*/
            file
        }
        Err(error) => {
            panic!("Failed to parse kernel file: {:?}", error);
        }
    };
    let entry_point = file.ehdr.e_entry;

    let (load_first_addr, load_last_addr) = calc_load_size(&file);
    let load_page_size = calc_size_in_pages_from_bytes((load_last_addr - load_first_addr) as usize);

    let physical_addr = match boot_services.allocate_pages(
        uefi::table::boot::AllocateType::Address(load_first_addr),
        uefi::table::boot::MemoryType::LOADER_DATA,
        load_page_size,
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

    let graphics_info = get_graphics_info(boot_services);

    let kernel_stack = match boot_services.allocate_pages(
        uefi::table::boot::AllocateType::AnyPages,
        uefi::table::boot::MemoryType::LOADER_DATA,
        calc_size_in_pages_from_bytes(1024 * 1024),
    ) {
        Ok(kernel_stack) => kernel_stack,
        Err(error) => {
            panic!("Failed to allocate pages: {:?}", error);
        }
    };

    let kernel_stack = unsafe { core::slice::from_raw_parts_mut(kernel_stack as *mut u8, 1024 * 1024) };
    let new_rsp = kernel_stack.as_ptr() as u64 + 1024 * 1024;

    let mut memmap = CommonMemoryMap{
        buffer: [MemoryDescriptor::default();256],
        length: 0,
    };
    

    drop(file_protocol);

    let (_, memory_map) = system_table.exit_boot_services();
    for descriptor in memory_map.entries().into_iter() {
        memmap.buffer[memmap.length] = *descriptor;
        memmap.length += 1;
    }

    unsafe {
        // ABIが違う
        let entry_point: extern "sysv64" fn(graphics_info: &GraphicsInfo, 
            memmap: &CommonMemoryMap, 
            new_rsp: u64) -> ! =
          core::mem::transmute(entry_point);
        entry_point(&graphics_info, 
            &memmap,
            new_rsp);
    }

    #[allow(unreachable_code)]
    Status::SUCCESS
}

fn get_graphics_info(boot_services: &BootServices) -> GraphicsInfo {
    let gop_handle =
        match boot_services.locate_handle_buffer(SearchType::from_proto::<GraphicsOutput>()) {
            Ok(handle) => handle,
            Err(err) => {
                panic!("Failed to locate_handle_buffer, {:?}", err);
            }
        };

    let mut gop = match boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle[0]) {
        Ok(gop) => gop,
        Err(err) => {
            panic!("Failed to open_protocol_exclusive, {:?}", err);
        }
    };
    let mode = gop.current_mode_info();
    let pixel_format = match mode.pixel_format() {
        uefi::proto::console::gop::PixelFormat::Rgb => PixelFormat::Rgb,
        uefi::proto::console::gop::PixelFormat::Bgr => PixelFormat::Bgr,
        _ => panic!("Unsupported pixel format"),
    };
    let mut frame_buffer = gop.frame_buffer();
    let graphics_info: GraphicsInfo = GraphicsInfo::new(
        mode.resolution().0,
        mode.resolution().1,
        mode.stride(),
        frame_buffer.as_mut_ptr(),
        pixel_format,
    );
    graphics_info
}

fn calc_load_size(file: &ElfBytes<AnyEndian>) -> (u64, u64) {
    let mut first_addr = u64::MAX;
    let mut last_addr = u64::MIN;
    for program_header in file.segments().unwrap() {
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
    (first_addr, last_addr)
}

fn calc_size_in_pages_from_bytes(bytes: usize) -> usize {
    (bytes + 0xfff) / 0x1000
}

fn pretty_print_memory_map(memory_map: &MemoryMap) {
    for descriptor in memory_map.entries() {
        uefi_services::println!(
            "addr: [{:#010x} - {:#010x}], len: {:#06} KiB, type: {:?}",
            descriptor.phys_start,
            descriptor.phys_start + descriptor.page_count * 4 * 1024 - 1,
            descriptor.page_count * 4,
            descriptor.ty
        );
    }
}
