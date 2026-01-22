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

use crate::default;

/// A simplified builder for log4rs configurations.
#[derive(Debug)]
pub struct ConfigBuilder {
    pub root_log_level: LevelFilter,
    pub log_levels: HashMap<String, LevelFilter>,
    pub appenders: HashMap<String, Box<dyn Append>>,
    pub filters: HashMap<String, Vec<Box<dyn Filter>>>,
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
    pub fn with_log_level(mut self, name: impl Into<String>, level: LevelFilter) -> Self {
        self.log_levels.insert(name.into(), level);
        self
    }

    /// Adds an appender to the configuration.
    pub fn with_appender(mut self, name: impl Into<String>, appender: Box<dyn Append>) -> Self {
        self.appenders.insert(name.into(), appender);
        self
    }

    /// Adds [`default::console_appender`] as "stdout".
    pub fn with_stdout_console_appender(self) -> Self {
        let console_appender = default::console_appender();
        self.with_appender("stdout", Box::new(console_appender))
    }

    /// Adds [`default::rolling_file_appender`] as "file".
    pub fn with_file_rolling_appender(self, path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let rolling_file_appender = default::rolling_file_appender(path)?;
        Ok(self.with_appender("file", Box::new(rolling_file_appender)))
    }

    /// Adds a filter to the configuration.
    pub fn with_filter(mut self, name: impl Into<String>, filter: Box<dyn Filter>) -> Self {
        self.filters.entry(name.into()).or_default().push(filter);
        self
    }

    /// Builds the [`Config`] from the provided settings.
    pub fn build(mut self) -> Result<Config, ConfigErrors> {
        let appender_names = self.appenders.keys().cloned().collect::<Vec<String>>();

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
        }

        for (name, level) in self.log_levels {
            builder = builder.logger(Logger::builder().build(name.as_str(), level));
        }

        let config = builder.build(
            Root::builder()
                .appenders(
                    appender_names
                        .iter()
                        .map(String::as_str)
                        .collect::<Vec<&str>>(),
                )
                .build(self.root_log_level),
        )?;

        Ok(config)
    }
}
