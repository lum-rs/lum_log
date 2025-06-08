/// Logs a message at the error level.
/// If the logger is not set up, the message is printed to stderr.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::is_set_up() {
            $crate::log::error!($($arg)*);
        } else {
            eprintln!($($arg)*);
        }
    };
}

/// Logs a message at the warn level.
/// If the logger is not set up, the message is printed to stdout.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::is_set_up() {
            $crate::log::warn!($($arg)*);
        } else {
            println!($($arg)*);
        }
    };
}

/// Logs a message at the info level.
/// If the logger is not set up, the message is printed to stdout.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::is_set_up() {
            $crate::log::info!($($arg)*);
        } else {
            println!($($arg)*);
        }
    };
}

/// Logs a message at the debug level.
/// If the logger is not set up, the message is printed to stdout.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::is_set_up() {
            $crate::log::debug!($($arg)*);
        } else {
            println!($($arg)*);
        }
    };
}

/// Logs a message at the trace level.
/// If the logger is not set up, the message is printed to stdout.
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if $crate::is_set_up() {
            $crate::log::trace!($($arg)*);
        } else {
            println!($($arg)*);
        }
    };
}
