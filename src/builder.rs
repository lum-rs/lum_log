use std::{fmt::Arguments, io};

use lum_libs::{
    fern::{self, colors::ColoredLevelConfig, FormatCallback},
    log::{LevelFilter, Record, SetLoggerError},
};

use crate::{defaults, logger, Config};

/// A `Builder` for configuring a logger and applying it as the global logger.
///
/// # Examples
/// ```
/// use std::{collections::HashMap, io};
/// use lum_log::{Builder, Config, defaults};
/// use lum_libs::log::LevelFilter;
///
/// let mut colors = HashMap::new();
/// colors.insert(LevelFilter::Info, "White".into());
/// colors.insert(LevelFilter::Error, "Red".into());
/// colors.insert(LevelFilter::Warn, "Yellow".into());
/// colors.insert(LevelFilter::Debug, "Green".into());
/// colors.insert(LevelFilter::Trace, "Blue".into());
///
/// let min_log_level = LevelFilter::Info;
///
/// let config = Config {
///     colors,
///     min_log_level,
/// };
///
/// let module_levels = [
///     ("some_lib".into(), LevelFilter::Debug),
///     ("some_other_lib".into(), LevelFilter::Trace),
///     ("some_other_lib::module".into(), LevelFilter::Warn),
/// ];
///
/// let result = Builder::new(defaults::format())
///     .config(&config)
///     .chain(io::stdout())
///     .is_debug_build(true)
///     .module_levels(&module_levels)
///     .apply();
///
/// assert!(result.is_ok());
/// ```
#[derive(Debug)]
pub struct Builder<'config, 'module_levels, FernOutput: Into<fern::Output>, FormatFn>
where
    Vec<FernOutput>: From<Vec<io::Stdout>>,
    FormatFn: Fn(FormatCallback, &Arguments, &Record, &ColoredLevelConfig) + Sync + Send + 'static,
{
    config: Option<&'config Config>,
    module_levels: Option<&'module_levels [(String, LevelFilter)]>,
    chains: Option<Vec<FernOutput>>,
    format: FormatFn,
    is_debug_build: bool,
}

impl<'config, 'module_levels, FernOutput: Into<fern::Output>, FormatFn>
    Builder<'config, 'module_levels, FernOutput, FormatFn>
where
    Vec<FernOutput>: From<Vec<io::Stdout>>,
    FormatFn: Fn(FormatCallback, &Arguments, &Record, &ColoredLevelConfig) + Sync + Send + 'static,
{
    /// Creates a new [`Builder`] with the given format.
    /// If you want to use the default format, use [`defaults::format()`](crate::defaults::format()).
    ///
    /// # Examples
    /// ```
    /// use lum_log::{Builder, defaults};
    ///
    /// let result = Builder::new(defaults::format()).apply();
    ///
    /// assert!(result.is_ok());
    /// ```
    pub fn new(format: FormatFn) -> Self {
        Self {
            config: None,
            module_levels: None,
            chains: None,
            format,
            is_debug_build: false,
        }
    }

    /// Sets the configuration for the logger.
    /// If you want to use the default configuration, do not call this method.
    pub fn config(self, config: &'config Config) -> Self {
        Self {
            config: Some(config),
            ..self
        }
    }

    /// Sets the module levels for the logger.
    /// If you want to use the default module levels, do not call this method.
    pub fn module_levels(self, module_levels: &'module_levels [(String, LevelFilter)]) -> Self {
        Self {
            module_levels: Some(module_levels),
            ..self
        }
    }

    /// Sets the chains for the logger.
    /// If you want to use the default chains, do not call this method.
    pub fn chains(self, chains: Vec<FernOutput>) -> Self {
        Self {
            chains: Some(chains),
            ..self
        }
    }

    /// Adds a chain to the logger.
    pub fn chain(self, chain: FernOutput) -> Self {
        let mut chains = self.chains.unwrap_or_default();
        chains.push(chain);

        Self {
            chains: Some(chains),
            ..self
        }
    }

    /// Sets the format for the logger.
    /// This overrides the format set in [`Builder::new`](Self::new).
    pub fn format(self, format: FormatFn) -> Self {
        Self { format, ..self }
    }

    /// Sets whether the logger is in debug build mode.
    ///
    /// # Examples
    /// ```
    /// use lum_log::{Builder, defaults};
    ///
    /// let is_debug_build = cfg!(debug_assertions);
    /// let result = Builder::new(defaults::format()).is_debug_build(is_debug_build).apply();
    ///
    /// assert!(result.is_ok());
    /// ```
    pub fn is_debug_build(self, is_debug_build: bool) -> Self {
        Self {
            is_debug_build,
            ..self
        }
    }

    /// Calls [`lum_log::setup`](crate::setup) with the given configuration to apply as the global logger.
    /// Optional fields that were not set will use the default values from [`defaults`].
    /// This can only be called once.
    pub fn apply(self) -> Result<(), SetLoggerError> {
        let config = match self.config {
            Some(config) => config,
            None => &defaults::config(),
        };

        let colors = &config.colors;
        let min_log_level = &config.min_log_level;

        let module_levels = self.module_levels.unwrap_or_default();

        let chains = match self.chains {
            Some(chains) => chains,
            None => defaults::chains().into(),
        };

        let format = self.format;
        let is_debug_build = self.is_debug_build;

        logger::setup(
            colors,
            min_log_level,
            module_levels,
            chains,
            format,
            &is_debug_build,
        )
    }
}
