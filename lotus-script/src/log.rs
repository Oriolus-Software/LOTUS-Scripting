//! Logging utilities.
use crate::{ffi, FfiObject};

/// Log level
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

/// Write a message with the given level. This is a low-level function, use the [log!], [debug!], [info!], [warning!], and [error!] macros instead.
pub fn write(level: Level, message: impl AsRef<str>) {
    let level = match level {
        Level::Debug => 0,
        Level::Info => 1,
        Level::Warn => 2,
        Level::Error => 3,
    };

    let message = FfiObject::new(&message.as_ref());
    unsafe {
        ffi::log::write(level, message.packed());
    }
}

/// Log a message with the given level.
#[doc(hidden)]
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::log::write($level, format!($($arg)*));
    };
}

/// Log a debug message.
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log!($crate::log::Level::Debug, $($arg)*);
    };
}

/// Log a info message.
#[doc(hidden)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!($crate::log::Level::Info, $($arg)*);
    };
}

/// Log a warning message.
#[doc(hidden)]
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        log!($crate::log::Level::Warn, $($arg)*);
    };
}

/// Log an error message.
#[doc(hidden)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log!($crate::log::Level::Error, $($arg)*);
    };
}

#[doc(inline)]
pub use debug;
#[doc(inline)]
pub use error;
#[doc(inline)]
pub use info;
#[doc(inline)]
pub use log;
#[doc(inline)]
pub use warning;
