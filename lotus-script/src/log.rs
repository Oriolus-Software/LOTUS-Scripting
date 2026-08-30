//! Logging-Hilfsfunktionen für Scripts.
//!
//! Logging utilities for scripts.

/// Log-Level für Script-Ausgaben.
///
/// Log level for script output.
pub enum Level {
    /// Debug-Ausgaben.
    ///
    /// Debug output.
    Debug,
    /// Informationsmeldungen.
    ///
    /// Informational messages.
    Info,
    /// Warnungen.
    ///
    /// Warnings.
    Warn,
    /// Fehlermeldungen.
    ///
    /// Error messages.
    Error,
}

/// Schreibt eine Meldung mit dem angegebenen Level.
/// Niedrigstufige Funktion — bevorzugt die Makros [`log!`], [`debug!`], [`info!`], [`warning!`] und [`error!`].
///
/// Writes a message with the given level.
/// Low-level function; prefer the [`log!`], [`debug!`], [`info!`], [`warning!`], and [`error!`] macros.
pub fn write(level: Level, message: impl AsRef<str>) {
    let level = match level {
        Level::Debug => 0,
        Level::Info => 1,
        Level::Warn => 2,
        Level::Error => 3,
    };

    let message = FfiObject::new(&message.as_ref());
    unsafe {
        lotus_script_sys::log::write(level, message.packed());
    }
}

/// Protokolliert eine Meldung mit dem angegebenen Level.
///
/// Logs a message with the given level.
#[doc(hidden)]
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::log::write($level, format!($($arg)*));
    };
}

/// Protokolliert eine Debug-Meldung.
///
/// Logs a debug message.
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log!($crate::log::Level::Debug, $($arg)*);
    };
}

/// Protokolliert eine Info-Meldung.
///
/// Logs an info message.
#[doc(hidden)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!($crate::log::Level::Info, $($arg)*);
    };
}

/// Protokolliert eine Warnmeldung.
///
/// Logs a warning message.
#[doc(hidden)]
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        log!($crate::log::Level::Warn, $($arg)*);
    };
}

/// Protokolliert eine Fehlermeldung.
///
/// Logs an error message.
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
use lotus_script_sys::FfiObject;
#[doc(inline)]
pub use warning;
