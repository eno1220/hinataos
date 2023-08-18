pub mod console;
pub mod frame_buffer;

use crate::graphics::{console::Console, frame_buffer::PixelInfo};
use common::types::GraphicsInfo;

use core::cell::OnceCell;
use spin::Mutex;

pub static mut CONSOLE: OnceCell<Mutex<Console<PixelInfo>>> = OnceCell::new();

pub fn init(graphics_info: &GraphicsInfo) {
    // todo(eno1220): 名称変更
    let pixel_info = PixelInfo {
        buffer: graphics_info.frame_buffer_base() as *mut u8,
        width: graphics_info.horizontal_resolution(),
        height: graphics_info.vertical_resolution(),
        pixels_per_line: graphics_info.pixels_per_scan_line(),
    };
    let console = Console::new(pixel_info, pixel_info.width / 8, pixel_info.height / 16);
    unsafe {
        CONSOLE.set(Mutex::new(console)).unwrap();
        CONSOLE.get().unwrap().lock().clear();
    }
}
