//use core::cell::RefCell;

//use crate::console::Console;
//use crate::graphics::PixelInfo;
use crate::serial::SerialPort;

// https://github.com/hikalium/wasabi/blob/main/os/src/print.rs
/*
pub struct GlobalPointer {
    console: RefCell<Option<Console<PixelInfo>>>,
}

impl GlobalPointer {
    pub fn set_console(&self, console: Console<PixelInfo>) {
        self.console.replace(Some(console));
        self.console.borrow_mut().as_mut().unwrap().clear();
    }
}

unsafe impl Sync for GlobalPointer {}

pub static GLOBAL_POINTER: GlobalPointer = GlobalPointer {
    console: RefCell::new(None),
};*/

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut serial = SerialPort::default();
    serial.write_fmt(args).unwrap();
    /*match &mut *GLOBAL_POINTER.console.borrow_mut() {
        Some(console) => {
            console.write_fmt(args).unwrap();
        }
        None => {}
    }*/
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

struct SerialLogger;

impl log::Log for SerialLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            serial_println!(
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

static LOGGER: SerialLogger = SerialLogger;

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}
