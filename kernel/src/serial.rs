use x86_64::structures::port::{PortRead, PortWrite};

pub const IO_ADDR_COM1: u16 = 0x3F8;
pub const IO_ADDR_COM2: u16 = 0x2F8;

pub fn serial_init(io_addr: u16) {
    unsafe {
        // Disable all interrupts
        u8::write_to_port(io_addr + 1, 0x00);
        // Enable DLAB (set baud rate divisor)
        u8::write_to_port(io_addr + 3, 0x80);
        const BAUD_DIVISOR: u16 = 0x0001;
        // Set divisor to 3 (lo byte) 38400 baud
        u8::write_to_port(io_addr, (BAUD_DIVISOR & 0xFF) as u8);
        u8::write_to_port(io_addr + 1, (BAUD_DIVISOR >> 8) as u8);
        // 8 bits, no parity, one stop bit
        u8::write_to_port(io_addr + 3, 0x03);
        // Enable FIFO, clear them, with 14-byte threshold
        u8::write_to_port(io_addr + 2, 0xC7);
        // IRQs enabled, RTS/DSR set
        u8::write_to_port(io_addr + 4, 0x0B);
    }
}

pub struct SerialPort {
    io_addr: u16,
}

impl SerialPort {
    pub fn send_byte(&mut self, byte: u8) {
        unsafe {
            while u8::read_from_port(self.io_addr + 5) & 0x20 == 0 {}
            u8::write_to_port(self.io_addr, byte);
        }
    }
    pub fn send_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.send_byte(byte);
        }
    }
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut serial = Self::default();
        serial.send_string(s);
        Ok(())
    }
}

impl Default for SerialPort {
    fn default() -> Self {
        Self {
            io_addr: IO_ADDR_COM1,
        }
    }
}
