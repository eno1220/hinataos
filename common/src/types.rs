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