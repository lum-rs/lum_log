use std::{collections::HashMap, io, path::Path};

use lum_libs::{
    log::LevelFilter,
    log4rs::{
        Config,
        append::Append,
        config::{Appender, Logger, Root, runtime::ConfigErrors},
        filter::Filter,
    },
};
use thiserror::Error;

use crate::default;

/// Errors that can occur when building a configuration.
/// By wrapping possible errors in this type, a user does not need to handle multiple error types when building a configuration.
#[derive(Debug, Error)]
pub enum ConfigBuilderError {
    #[error("I/O error while creating rolling file appender: {0}")]
    FileRollingAppenderIo(#[from] io::Error),

    #[error("Error while building log4rs configuration: {0}")]
    Log4rs(#[from] ConfigErrors),
}

/// A simplified builder for log4rs configurations.
/// Note that this supports adding appenders to the root logger only.
#[derive(Debug)]
pub struct ConfigBuilder {
    root_log_level: LevelFilter,
    log_levels: HashMap<String, LevelFilter>,
    appenders: HashMap<String, Box<dyn Append>>,
    filters: HashMap<String, Vec<Box<dyn Filter>>>,
}

impl Default for ConfigBuilder {
    /// Creates a default `ConfigBuilder`, using the root log level from [`default::log_level`], no log levels, no appenders, and no filters.
    fn default() -> Self {
        Self {
            root_log_level: default::log_level(),
            log_levels: HashMap::new(),
            appenders: HashMap::new(),
            filters: HashMap::new(),
        }
    }
}

impl ConfigBuilder {
    /// Same as [`ConfigBuilder::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the log level of the root logger.
    pub fn root_log_level(mut self, level: LevelFilter) -> Self {
        self.root_log_level = level;
        self
    }

    /// Adds a log level for a specific logger name.
    pub fn log_level(mut self, name: impl Into<String>, level: LevelFilter) -> Self {
        self.log_levels.insert(name.into(), level);
        self
    }

    /// Adds an appender to the configuration.
    pub fn appender(mut self, name: impl Into<String>, appender: Box<dyn Append>) -> Self {
        self.appenders.insert(name.into(), appender);
        self
    }

    /// Adds [`default::console_appender`] as "stdout".
    pub fn stdout_console_appender(self) -> Self {
        let console_appender = default::console_appender();
        self.appender("stdout", Box::new(console_appender))
    }

    /// Adds [`default::rolling_file_appender`] as "file".
    pub fn file_rolling_appender(self, path: impl AsRef<Path>) -> Result<Self, ConfigBuilderError> {
        let rolling_file_appender = default::rolling_file_appender(path)?;
        Ok(self.appender("file", Box::new(rolling_file_appender)))
    }

    /// Adds a filter to the configuration.
    pub fn filter(mut self, name: impl Into<String>, filter: Box<dyn Filter>) -> Self {
        self.filters.entry(name.into()).or_default().push(filter);
        self
    }

    /// Builds the [`Config`] from the provided settings.
    pub fn build(mut self) -> Result<Config, ConfigBuilderError> {
        let mut appender_names = Vec::with_capacity(self.appenders.len());

        let mut builder = Config::builder();
        for (name, append) in self.appenders {
            let filters = self.filters.remove(&name);

            let mut appender = Appender::builder();
            if let Some(filters) = filters {
                for filter in filters {
                    appender = appender.filter(filter);
                }
            }
            let appender = appender.build(name.as_str(), append);

            builder = builder.appender(appender);
            appender_names.push(name);
        }

        for (name, level) in self.log_levels {
            builder = builder.logger(Logger::builder().build(name.as_str(), level));
        }

        let config = builder.build(
            Root::builder()
                .appenders(appender_names)
                .build(self.root_log_level),
        )?;

        Ok(config)
    }
}
