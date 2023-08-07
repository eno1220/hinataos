use uefi::table::boot::{MemoryDescriptor, MemoryType};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GraphicsInfo {
    horizontal_resolution: usize,
    vertical_resolution: usize,
    pixels_per_scan_line: usize,
    frame_buffer_base: *mut u8,
    pixel_format: PixelFormat,
}

impl GraphicsInfo {
    pub fn new(
        horizontal_resolution: usize,
        vertical_resolution: usize,
        pixels_per_scan_line: usize,
        frame_buffer_base: *mut u8,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            horizontal_resolution,
            vertical_resolution,
            pixels_per_scan_line,
            frame_buffer_base,
            pixel_format,
        }
    }

    pub fn horizontal_resolution(&self) -> usize {
        self.horizontal_resolution
    }

    pub fn vertical_resolution(&self) -> usize {
        self.vertical_resolution
    }

    pub fn pixels_per_scan_line(&self) -> usize {
        self.pixels_per_scan_line
    }

    pub fn frame_buffer_base(&self) -> *mut u8 {
        self.frame_buffer_base
    }

    pub fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }
}

pub struct MemoryMap{
    pub buffer: [MemoryDescriptor; 256],
    pub length: usize,
}

pub fn is_available(memory_type: MemoryType) -> bool {
    match memory_type{
        MemoryType::BOOT_SERVICES_CODE => true,
        MemoryType::BOOT_SERVICES_DATA => true,
        MemoryType::CONVENTIONAL => true,
        _ => false,
    }
}