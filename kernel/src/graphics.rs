use common::types::GraphicsInfo;
use spin::{Lazy, Mutex};
use crate::font::FONT;
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct PixelWriter {
    graphics_info: GraphicsInfo,
    write: fn(&mut Self, x: usize, y: usize, color: &Color),
}

impl PixelWriter {
    pub fn new(graphics_info: GraphicsInfo) -> Self {
        let write = match graphics_info.pixel_format() {
            common::types::PixelFormat::Rgb => Self::write_rgb,
            common::types::PixelFormat::Bgr => Self::write_bgr,
        };
        Self {
            graphics_info,
            write,
        }
    }

    fn write_rgb(&mut self, x: usize, y: usize, color: &Color) {
        let offset = (y * self.graphics_info.pixels_per_scan_line() + x) * 4;
        unsafe {
            self.graphics_info
                .frame_buffer_base()
                .add(offset)
                .write_volatile(color.r);
            self.graphics_info
                .frame_buffer_base()
                .add(offset + 1)
                .write_volatile(color.g);
            self.graphics_info
                .frame_buffer_base()
                .add(offset + 2)
                .write_volatile(color.b);
        }
    }

    fn write_bgr(&mut self, x: usize, y: usize, color: &Color) {
        let offset = (y * self.graphics_info.pixels_per_scan_line() + x) * 4;
        unsafe {
            self.graphics_info
                .frame_buffer_base()
                .add(offset)
                .write_volatile(color.b);
            self.graphics_info
                .frame_buffer_base()
                .add(offset + 1)
                .write_volatile(color.g);
            self.graphics_info
                .frame_buffer_base()
                .add(offset + 2)
                .write_volatile(color.r);
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        (self.write)(self, x, y, color);
    }

    fn write_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: &Color) {
        for dy in 0..height {
            for dx in 0..width {
                self.write_pixel(x + dx, y + dy, color);
            }
        }
    }

    fn fill(&mut self, color: &Color) {
        self.write_rect(
            0,
            0,
            self.graphics_info.horizontal_resolution(),
            self.graphics_info.vertical_resolution(),
            color,
        );
    }

    fn clear(&mut self) {
        self.fill(&Color { r: 0, g: 0, b: 0 });
    }

    fn write_ascii(&mut self, x: usize, y: usize, c: char, color: &Color) {
        let font = &FONT[c as usize];
        for dy in 0..16 {
            for dx in 0..8 {
                if (font[dy] >> (7 - dx)) & 1 == 1 {
                    self.write_pixel(x + dx, y + dy, color);
                }
            }
        }
    }
}

pub static mut PIXEL_WRITER: Lazy<Mutex<Option<PixelWriter>>> = Lazy::new(|| Mutex::new(None));

pub fn graphics_init(graphics_info: GraphicsInfo) {
    let mut pixel_writer = unsafe { PIXEL_WRITER.lock() };
    *pixel_writer = Some(PixelWriter::new(graphics_info));
    pixel_writer.as_mut().unwrap().clear();
}

pub fn write_something() {
    let mut pixel_writer = unsafe { PIXEL_WRITER.lock() };
    pixel_writer.as_mut().unwrap().write_ascii(0, 0, 'H', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(8, 0, 'e', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(16, 0, 'l', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(24, 0, 'l', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(32, 0, 'o', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(40, 0, ',', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(48, 0, 'w', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(56, 0, 'o', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(64, 0, 'r', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(72, 0, 'l', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(80, 0, 'd', &Color { r: 255, g: 255, b: 255 });
    pixel_writer.as_mut().unwrap().write_ascii(88, 0, '!', &Color { r: 255, g: 255, b: 255 });

}



