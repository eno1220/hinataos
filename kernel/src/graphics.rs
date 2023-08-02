// pixel_formatがBgr形式であることを前提としているので注意
use crate::font::FONT;

pub trait PixelBuffer {
    fn buffer(&self) -> *mut u8;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels_per_line(&self) -> usize;
    fn calc_offset(&self, x: usize, y: usize) -> usize {
        (self.pixels_per_line() * y + x) as usize * 4
    }
    fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PixelInfo {
    pub buffer: *mut u8,
    pub width: usize,
    pub height: usize,
    pub pixels_per_line: usize,
}

impl PixelBuffer for PixelInfo {
    fn buffer(&self) -> *mut u8 {
        self.buffer
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn pixels_per_line(&self) -> usize {
        self.pixels_per_line
    }
}

pub fn write_pixel<T: PixelBuffer>(buffer: &mut T, x: usize, y: usize, color: u32) {
    if !buffer.is_valid(x, y) {
        return;
    }
    unsafe {
        let pixel = buffer.buffer().add(buffer.calc_offset(x, y));
        pixel.write_volatile(color as u8);
        pixel.add(1).write_volatile((color >> 8) as u8);
        pixel.add(2).write_volatile((color >> 16) as u8);
    }
}

pub fn write_rect<T: PixelBuffer>(
    buffer: &mut T,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    if !buffer.is_valid(x, y) && !buffer.is_valid(x + width - 1, y + height - 1) {
        return;
    }
    for i in 0..height {
        for j in 0..width {
            write_pixel(buffer, x + j, y + i, color);
        }
    }
}

pub fn write_char<T: PixelBuffer>(buffer: &mut T, x: usize, y: usize, color: u32, c: char) {
    if !buffer.is_valid(x, y) {
        return;
    }
    let font = FONT[c as usize];
    for i in 0..16 {
        for j in 0..8 {
            if font[i] & 1 << (7 - j) != 0 {
                write_pixel(buffer, x + j, y + i, color);
            }
        }
    }
}

pub fn transfer_rect<T: PixelBuffer>(
    src: &mut T,
    src_x: usize,
    src_y: usize,
    dest_x: usize,
    dest_y: usize,
    width: usize,
    height: usize,
) {
    if !src.is_valid(src_x, src_y) && !src.is_valid(src_x + width - 1, src_y + height - 1) {
        return;
    }
    for i in 0..height {
        unsafe {
            let src_pixel = src.buffer().add(src.calc_offset(src_x, src_y + i));
            let dest_pixel = src.buffer().add(src.calc_offset(dest_x, dest_y + i));
            core::ptr::copy_nonoverlapping(src_pixel, dest_pixel, width * 4);
        }
    }
}