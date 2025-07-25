/// Logs a message at the error level.
/// If the logger is not set up, the message is printed to stderr.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::is_set_up() {
            $crate::log::error!($($arg)*);
        } else {
            std::eprintln!($($arg)*);
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
            std::println!($($arg)*);
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
            std::println!($($arg)*);
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
            std::println!($($arg)*);
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
            std::println!($($arg)*);
        }
    };
}

/// Calls the `error!` macro and then panics by using the `panic!` macro with the same message.
#[macro_export]
macro_rules! error_panic {
    ($($arg:tt)*) => {
        $crate::error!($($arg)*);
        std::panic!($($arg)*);
    };
}

/// Calls the `error!` macro and then panics by using the `unreachable!` macro with the same message.
#[macro_export]
macro_rules! error_unreachable {
    ($($arg:tt)*) => {
        $crate::error!($($arg)*);
        std::unreachable!($($arg)*);
    };
}
