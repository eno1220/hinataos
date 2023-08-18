use crate::graphics::{transfer_rect, write_char, write_rect, PixelBuffer};


pub struct Console<T: PixelBuffer>{
    buf: T,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    cursor_x: usize,
    cursor_y: usize,
}

impl<T: PixelBuffer> Console<T>{
    pub fn new(buf:T,w:usize,h:usize)->Self{
        Self{
            buf,
            x:16,
            y:8,
            w,
            h,
            cursor_x:0,
            cursor_y:0,
        }
    }

    fn new_line(&mut self){
        self.cursor_x = 0;
        if (self.cursor_y - 1) >= self.h{
            self.cursor_y = self.h - 1;
            for row in 1..self.h{
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
                self.x * self.y,
                self.y,
                0x000000,
            );
        }else {
            self.cursor_y += 1;
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

impl <T: PixelBuffer> core::fmt::Write for Console<T>{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_string(s);
        Ok(())
    }
}

/*
pub struct Console<T: PixelBuffer> {
    buffer: T,
    width: usize,
    height: usize,
    pixels_per_line: usize,
    cursor_x: usize,
    cursor_y: usize,
    text_buffer: [[char; CONSOLE_WIDTH]; CONSOLE_HEIGHT],
    log_level: log::LevelFilter,
}
*/

/*
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
            log_level: log::LevelFilter::Info,
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
        } else if c == '\r' {
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
}*/
