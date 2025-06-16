use std::{collections::HashMap, fmt::Arguments, io};

use lum_libs::{
    fern::{
        self, FormatCallback,
        colors::{Color, ColoredLevelConfig},
    },
    log::{LevelFilter, Record, SetLoggerError},
};

use crate::{defaults, logger};

/// A `Builder` for configuring a logger and applying it as the global logger.
///
/// # Examples
/// ```
/// use std::{collections::HashMap, io};
/// use lum_log::{Builder, defaults};
/// use lum_libs::{log::LevelFilter, fern::colors::Color};
///
/// let result = Builder::new(defaults::format())
///     .color(LevelFilter::Error, Color::Red)
///     .color(LevelFilter::Warn, Color::Yellow)
///     .color(LevelFilter::Info, Color::Green)
///     .color(LevelFilter::Debug, Color::Magenta)
///     .color(LevelFilter::Trace, Color::Cyan)
///     .min_log_level(LevelFilter::Info)
///     .module_level("some_lib", LevelFilter::Debug)
///     .module_level("some_other_lib", LevelFilter::Trace)
///     .module_level("some_other_lib::module", LevelFilter::Warn)
///     .output(io::stdout())
///     .is_debug_build(true)
///     .apply();
///
/// assert!(result.is_ok());
/// ```
#[derive(Debug)]
pub struct Builder<FernOutput: Into<fern::Output>, FormatFn>
where
    Vec<FernOutput>: From<Vec<io::Stdout>>,
    FormatFn: Fn(FormatCallback, &Arguments, &Record, &ColoredLevelConfig) + Sync + Send + 'static,
{
    colors: Option<HashMap<LevelFilter, Color>>,
    min_log_level: Option<LevelFilter>,
    module_levels: HashMap<String, LevelFilter>,
    outputs: Option<Vec<FernOutput>>,
    format: FormatFn,
    is_debug_build: bool,
}

impl<FernOutput: Into<fern::Output>, FormatFn> Builder<FernOutput, FormatFn>
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
            colors: None,
            min_log_level: None,
            module_levels: HashMap::new(),
            outputs: None,
            format,
            is_debug_build: false,
        }
    }

    /// Sets the per-level colors.
    /// If you want to use the default colors, do not call this method.
    pub fn colors(self, colors: HashMap<LevelFilter, Color>) -> Self {
        Self {
            colors: Some(colors),
            ..self
        }
    }

    /// Sets a color for a specific log level.
    /// If you want to use the default colors, do not call this method.
    pub fn color(self, level: LevelFilter, color: Color) -> Self {
        let mut colors = self.colors.unwrap_or_default();
        colors.insert(level, color);

        Self {
            colors: Some(colors),
            ..self
        }
    }

    /// Sets the minimum log level.
    /// If you want to use the default minimum log level, do not call this method.
    pub fn min_log_level(self, min_log_level: LevelFilter) -> Self {
        Self {
            min_log_level: Some(min_log_level),
            ..self
        }
    }

    /// Sets the module levels for the logger.
    /// By default, there are no module levels set.
    pub fn module_levels(self, module_levels: HashMap<String, LevelFilter>) -> Self {
        Self {
            module_levels,
            ..self
        }
    }

    /// Sets the module level for a specific module.
    /// By default, there are no module levels set.
    pub fn module_level(self, module: impl Into<String>, level: LevelFilter) -> Self {
        let module = module.into();

        let mut module_levels = self.module_levels;
        module_levels.insert(module, level);

        Self {
            module_levels,
            ..self
        }
    }

    /// Sets the chains for the logger.
    /// If you want to use the default outputs, do not call this method.
    pub fn outputs(self, outputs: Vec<FernOutput>) -> Self {
        Self {
            outputs: Some(outputs),
            ..self
        }
    }

    /// Adds a chain to the logger.
    /// If you want to use the default outputs, do not call this method.
    pub fn output(self, output: FernOutput) -> Self {
        let mut outputs = self.outputs.unwrap_or_default();
        outputs.push(output);

        Self {
            outputs: Some(outputs),
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
        let colors = match self.colors {
            Some(colors) => colors,
            None => defaults::colors(),
        };

        let min_log_level = match self.min_log_level {
            Some(min_log_level) => min_log_level,
            None => defaults::min_log_level(),
        };

        let module_levels = self.module_levels;

        let outputs = match self.outputs {
            Some(outputs) => outputs,
            None => defaults::outputs().into(),
        };

        let format = self.format;
        let is_debug_build = self.is_debug_build;

        let colors = colors.into_iter().collect::<Vec<_>>();
        let module_levels = module_levels.into_iter().collect::<Vec<_>>();

        logger::setup(
            &colors,
            &min_log_level,
            &module_levels,
            outputs,
            format,
            &is_debug_build,
        )
    }
}
