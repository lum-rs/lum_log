use std::{
    io::{self},
    path::Path,
};

use lum_libs::{
    log::LevelFilter,
    log4rs::{
        append::{
            console::ConsoleAppender,
            rolling_file::{
                RollingFileAppender,
                policy::compound::{
                    CompoundPolicy,
                    roll::fixed_window::FixedWindowRoller,
                    trigger::time::{TimeTrigger, TimeTriggerConfig, TimeTriggerInterval},
                },
            },
        },
        encode::pattern::PatternEncoder,
    },
};

/// Returns the log level [`LevelFilter::Info`].
pub fn log_level() -> LevelFilter {
    LevelFilter::Info
}

/// Returns a general-purpose log format string.
/// The format resolves to the following:
/// ```text
/// [2024-11-12T21:10:32Z main example::module::path INFO ] This is a log message
/// ```
pub fn format() -> &'static str {
    "[{d(%Y-%m-%d %H:%M:%S%.3f)} {T:<-10.10} {t:<-40.40} {h({l:<5})}] {m}{n}"
}

/// Returns a [`ConsoleAppender`] with a [`PatternEncoder`] using the format returned by [`format()`].
pub fn console_appender() -> ConsoleAppender {
    ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(format())))
        .build()
}

/// Returns a [`TimeTriggerConfig`] with daily rolling, modulated, and no random delay.
pub fn time_trigger_config() -> TimeTriggerConfig {
    TimeTriggerConfig {
        interval: TimeTriggerInterval::Day(1),
        modulate: true,
        max_random_delay: 0,
    }
}

/// Returns a [`RollingFileAppender`] with a [`PatternEncoder`]
/// using the format returned by [`format()`],
/// and the [`TimeTriggerConfig`] provided by [`time_trigger_config()`],
/// writing to the given path.
pub fn rolling_file_appender(path: impl AsRef<Path>) -> io::Result<RollingFileAppender> {
    RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(format())))
        .build(
            path,
            Box::new(CompoundPolicy::new(
                Box::new(TimeTrigger::new(time_trigger_config())),
                Box::new(
                    FixedWindowRoller::builder()
                        .base(0)
                        .build("{}.log", 10)
                        .expect("Hard-coded example should always build successfully"),
                ),
            )),
        )
}

/// Returns a tuple of the [`ConsoleAppender`] and [`RollingFileAppender`]
/// returned by [`console_appender`] and [`rolling_file_appender`], respectively.
pub fn appenders(path: impl AsRef<Path>) -> (ConsoleAppender, io::Result<RollingFileAppender>) {
    let console_appender = console_appender();
    let rolling_file_appender = rolling_file_appender(path);

    (console_appender, rolling_file_appender)
}
