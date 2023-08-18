use crate::graphics::frame_buffer::{transfer_rect, write_char, write_rect, PixelBuffer};

#[derive(Debug)]
pub struct Console<T: PixelBuffer> {
    buf: T,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    cursor_x: usize,
    cursor_y: usize,
}

impl<T: PixelBuffer> Console<T> {
    pub fn new(buf: T, w: usize, h: usize) -> Self {
        Self {
            buf,
            x: 8,
            y: 16,
            w,
            h,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn clear(&mut self) {
        write_rect(
            &mut self.buf,
            0,
            0,
            self.x * self.w,
            self.y * self.h,
            0x000000,
        );
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y >= self.h {
            self.cursor_y = self.h - 1;
            for row in 1..self.h {
                transfer_rect(
                    &mut self.buf,
                    0,
                    row * self.y,
                    0,
                    (row - 1) * self.y,
                    self.x * self.w,
                    self.y,
                );
            }
            write_rect(
                &mut self.buf,
                0,
                (self.h - 1) * self.y,
                self.x * self.w,
                self.y,
                0x000000,
            );
        }
    }

    // todo(eno1220): 色を指定できるようにする
    pub fn print_char(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
        } else if c == '\r' {
        } else {
            write_char(
                &mut self.buf,
                self.cursor_x * self.x,
                self.cursor_y * self.y,
                0xffffff,
                c,
            );
            self.cursor_x += 1;
            if self.cursor_x >= self.w {
                self.new_line();
            }
        }
    }

    pub fn print_string(&mut self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }
}

impl<T: PixelBuffer> core::fmt::Write for Console<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_string(s);
        Ok(())
    }
}
