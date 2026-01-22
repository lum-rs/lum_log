/// Logs a message at the error level.
/// If the logger is not set up, the message is printed to stderr.
/// **This macro uses a Mutex under the hood, so do not use it in performance-critical code.**
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
/// **This macro uses a Mutex under the hood, so do not use it in performance-critical code.**
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
/// **This macro uses a Mutex under the hood, so do not use it in performance-critical code.**
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
/// **This macro uses a Mutex under the hood, so do not use it in performance-critical code.**
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
/// **This macro uses a Mutex under the hood, so do not use it in performance-critical code.**
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
/// **This macro uses a Mutex under the hood, so do not use it in performance-critical code.**
#[macro_export]
macro_rules! error_panic {
    ($($arg:tt)*) => {
        $crate::error!($($arg)*);
        std::panic!($($arg)*);
    };
}

/// Calls the `error!` macro and then panics by using the `unreachable!` macro with the same message.
/// **This macro uses a Mutex under the hood, so do not use it in performance-critical code.**
#[macro_export]
macro_rules! error_unreachable {
    ($($arg:tt)*) => {
        $crate::error!($($arg)*);
        std::unreachable!($($arg)*);
    };
}
