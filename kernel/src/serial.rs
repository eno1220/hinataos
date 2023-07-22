use spin::Mutex;
use x86_64::structures::port::PortWrite;

struct SerialWriter();

impl SerialWriter {
    fn write_byte(&mut self, byte: u8) {
        unsafe {
            u8::write_to_port(0x3F8, byte);
        }
    }
}

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

static SERIAL_WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter());

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! serial_print{
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println{
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n",format_args!($($arg)*)));
}
