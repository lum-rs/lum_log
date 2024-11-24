use std::{
    collections::HashMap,
    fmt::Arguments,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

use lum_libs::{
    fern::{
        self,
        colors::{Color, ColoredLevelConfig},
        FormatCallback,
    },
    log::{LevelFilter, Record, SetLoggerError},
};

static IS_LOGGER_SET_UP: AtomicBool = AtomicBool::new(false);

/// Returns whether the logger has been set up.
/// This uses an atomic boolean under the hood, so it is safe for concurrent use.
pub fn is_set_up() -> bool {
    IS_LOGGER_SET_UP.load(Ordering::Relaxed)
}

/// Sets up the logger with the given configuration and applies it as the global logger.
/// This uses [`fern`] under the hood.
/// For a more intuitive way to set up the logger, see the [`Builder`](crate::Builder).
/// This can only be called once.
pub fn setup<FernOutput: Into<fern::Output>, FormatFn>(
    colors: &HashMap<LevelFilter, String>,
    min_log_level: &LevelFilter,
    module_levels: &[(String, LevelFilter)],
    chains: Vec<FernOutput>,
    format: FormatFn,
    is_debug_build: &bool,
) -> Result<(), SetLoggerError>
where
    FormatFn: Fn(FormatCallback, &Arguments, &Record, &ColoredLevelConfig) + Sync + Send + 'static,
{
    let mut color_config: ColoredLevelConfig = ColoredLevelConfig::new();
    for (level, color) in colors.iter() {
        let color = Color::from_str(color).unwrap_or(Color::White);
        match level {
            LevelFilter::Info => {
                color_config = color_config.info(color);
            }
            LevelFilter::Error => {
                color_config = color_config.error(color);
            }
            LevelFilter::Warn => {
                color_config = color_config.warn(color);
            }
            LevelFilter::Debug => {
                color_config = color_config.debug(color);
            }
            LevelFilter::Trace => {
                color_config = color_config.trace(color);
            }
            LevelFilter::Off => {}
        }
    }

    let min_log_level = match is_debug_build {
        true if *min_log_level != LevelFilter::Trace => LevelFilter::Debug,
        true => LevelFilter::Trace,
        false => *min_log_level,
    };

    let mut logger = fern::Dispatch::new()
        .level(min_log_level)
        .format(move |out, message, record| format(out, message, record, &color_config));

    for (module, level) in module_levels.iter() {
        logger = logger.level_for(module.clone(), *level);
    }

    for chain in chains {
        logger = logger.chain(chain);
    }

    logger.apply()?;

    IS_LOGGER_SET_UP.store(true, Ordering::Relaxed);
    Ok(())
}
