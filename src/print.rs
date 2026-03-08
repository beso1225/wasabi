use core::fmt;

use crate::serial::SerialPort;

pub fn global_print(args: fmt::Arguments) {
    let mut writer = SerialPort::default();
    fmt::write(&mut writer, args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        ($crate::print::global_print(format_args!($($args)*)));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
        ($($args:tt)*) => ($crate::print!("{}\n", format_args!($($args)*)));
}
