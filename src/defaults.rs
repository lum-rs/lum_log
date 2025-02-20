use std::{
    fmt::Arguments,
    io::{self},
    time::SystemTime,
};

use lum_libs::{
    fern::{FormatCallback, colors::ColoredLevelConfig},
    humantime,
    log::Record,
};

use crate::Config;

/// Returns a default `Config`.
/// For more information, see [`Config::default`](crate::Config::default).
pub fn config() -> Config {
    Config::default()
}

/// Returns a vector containing [`io::stdout()`](std::io::stdout).
/// Into [`fern::Dispatch`](lum_libs::fern::Dispatch) is implemented for Vec<io::Stdout>, so this can be used as a chain.
pub fn chains() -> Vec<io::Stdout> {
    vec![io::stdout()]
}

/// Returns a closure that formats the log message in the following format:
/// ```text
/// [2024-11-12T21:10:32Z example::module::path INFO ] This is a log message
/// ```
pub fn format()
-> impl Fn(FormatCallback, &Arguments, &Record, &ColoredLevelConfig) + Sync + Send + 'static {
    move |out: FormatCallback, message: &Arguments, record: &Record, colors: &ColoredLevelConfig| {
        out.finish(format_args!(
            "[{} {: <30} {: <5}] {}",
            humantime::format_rfc3339_seconds(SystemTime::now()),
            record.target(),
            colors.color(record.level()),
            message
        ))
    }
}
