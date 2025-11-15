use std::{
    io::{self},
    path::Path,
};

use lum_libs::{
    log::LevelFilter,
    log4rs::{
        Config,
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
        config::{Appender, Root, runtime::ConfigErrors},
        encode::pattern::PatternEncoder,
    },
};
use thiserror::Error;

/// Returns the default minimum log level [`LevelFilter::Info`]
pub fn min_log_level() -> LevelFilter {
    LevelFilter::Info
}

/// Returns the default log format string in log4rs format.
/// The format resolves to the following:
/// ```text
/// [2024-11-12T21:10:32Z main example::module::path INFO ] This is a log message
/// ```
pub fn format() -> &'static str {
    "[{d(%Y-%m-%d %H:%M:%S%.3f)} {T:<-10.10} {t:<-40.40} {h({l:<5})}] {m}{n}"
}
/// Returns a default [`ConsoleAppender`] with a given format.
pub fn console_appender(format: &str) -> ConsoleAppender {
    ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(format)))
        .build()
}

/// Returns a default [`TimeTriggerConfig`] with daily rolling, modulated, and no random delay.
pub fn time_trigger_config() -> TimeTriggerConfig {
    TimeTriggerConfig {
        interval: TimeTriggerInterval::Day(1),
        modulate: true,
        max_random_delay: 0,
    }
}

/// Returns a default [`RollingFileAppender`] with a given format and [`TimeTriggerConfig`].
pub fn rolling_file_appender(
    path: impl AsRef<Path>,
    format: &str,
    time_trigger_config: TimeTriggerConfig,
) -> io::Result<RollingFileAppender> {
    RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(format)))
        .build(
            path,
            Box::new(CompoundPolicy::new(
                Box::new(TimeTrigger::new(time_trigger_config)),
                Box::new(
                    FixedWindowRoller::builder()
                        .base(0)
                        .build("{}.log", 10)
                        .unwrap(),
                ),
            )),
        )
}

/// Returns the default [`ConsoleAppender`] and [`RollingFileAppender`] with a given format and [`TimeTriggerConfig`].
pub fn appenders(
    path: impl AsRef<Path>,
    format: &str,
    time_trigger_config: TimeTriggerConfig,
) -> (ConsoleAppender, io::Result<RollingFileAppender>) {
    let console_appender = console_appender(format);
    let rolling_file_appender = rolling_file_appender(path, format, time_trigger_config);

    (console_appender, rolling_file_appender)
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error creating rolling file appender: {0}")]
    RollingFileAppenderError(#[from] io::Error),

    #[error("Error building log4rs config: {0}")]
    ConfigBuildError(#[from] ConfigErrors),
}

/// Returns the default log4rs [`Config`] with console and rolling file appenders with a given format and [`TimeTriggerConfig`].
/// The config is meant to be a barebone, but ready-to-use logging configuration.
/// It includes a [`ConsoleAppender`] and a [`RollingFileAppender`] that rolls based on the provided [`TimeTriggerConfig`] on the given path.
pub fn config(
    path: impl AsRef<Path>,
    format: &str,
    time_trigger_config: TimeTriggerConfig,
) -> Result<Config, ConfigError> {
    let (console_appender, rolling_file_appender_result) =
        appenders(path, format, time_trigger_config);
    let rolling_file_appender = rolling_file_appender_result?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(console_appender)))
        .appender(Appender::builder().build("file", Box::new(rolling_file_appender)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Trace),
        )?;

    Ok(config)
}
