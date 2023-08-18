use crate::graphics::CONSOLE;
use crate::serial::SerialPort;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut serial = SerialPort::default();
    serial.write_fmt(args).unwrap();
    unsafe {
        CONSOLE.get().unwrap().lock().write_fmt(args).unwrap();
    }
}

#[macro_export]
macro_rules! print{
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println{
    () => ($crate::serial_print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n",format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _serial_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut serial = SerialPort::default();
    serial.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! serial_print{
    ($($arg:tt)*) => ($crate::print::_serial_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println{
    () => ($crate::serial_print!("\r\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\r\n",format_args!($($arg)*)));
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{:>5}]: {:>12}@{:>4}: {}",
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}
