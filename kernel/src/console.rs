use crate::graphics::{transfer_rect, write_char, write_pixel, write_rect, PixelBuffer};

pub const CONSOLE_WIDTH: usize = 40;
pub const CONSOLE_HEIGHT: usize = 25;
pub const ROW_HEIGHT: usize = 16;

pub struct Console<T: PixelBuffer> {
    buffer: T,
    width: usize,
    height: usize,
    pixels_per_line: usize,
    cursor_x: usize,
    cursor_y: usize,
    text_buffer: [[char; CONSOLE_WIDTH]; CONSOLE_HEIGHT],
}

// 適当実装なので直す
static mut CONSOLE_BUFFER: [[char; CONSOLE_WIDTH]; CONSOLE_HEIGHT] =
    [[' '; CONSOLE_WIDTH]; CONSOLE_HEIGHT];
impl<T: PixelBuffer> Console<T> {
    pub fn new(buffer: T, width: usize, height: usize, pixels_per_line: usize) -> Self {
        Self {
            buffer,
            width,
            height,
            pixels_per_line,
            cursor_x: 0,
            cursor_y: 0,
            text_buffer: unsafe { core::mem::transmute(CONSOLE_BUFFER) },
        }
    }
    fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y >= CONSOLE_HEIGHT {
            self.cursor_y = CONSOLE_HEIGHT - 1;

            for row in 1..CONSOLE_HEIGHT {
                for col in 0..CONSOLE_WIDTH {
                    let character = self.text_buffer[row][col];
                    self.text_buffer[row - 1][col] = character;
                    transfer_rect(
                        &mut self.buffer,
                        col * 8,
                        row * 16,
                        col * 8,
                        (row - 1) * 16,
                        8,
                        16,
                    );
                }
            }
            for col in 0..CONSOLE_WIDTH {
                self.text_buffer[CONSOLE_HEIGHT - 1][col] = ' ';
            }

            write_rect(
                &mut self.buffer,
                0,
                (CONSOLE_HEIGHT - 1) * ROW_HEIGHT,
                self.width,
                ROW_HEIGHT,
                0x000000,
            );
        }
    }
    pub fn print_char(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
        } else {
            write_char(
                &mut self.buffer,
                self.cursor_x * 8,
                self.cursor_y * 16,
                0xffffff,
                c,
            );
            self.text_buffer[self.cursor_y][self.cursor_x] = c;
            self.cursor_x += 1;
            if self.cursor_x >= CONSOLE_WIDTH {
                self.new_line();
            }
        }
    }
    pub fn print_string(&mut self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }
    pub fn clear(&mut self) {
        for row in 0..CONSOLE_HEIGHT {
            for col in 0..CONSOLE_WIDTH {
                self.text_buffer[row][col] = ' ';
            }
        }
        write_rect(&mut self.buffer, 0, 0, self.width, self.height, 0x000000);
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
}

impl<T: PixelBuffer> core::fmt::Write for Console<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_string(s);
        Ok(())
    }
}
