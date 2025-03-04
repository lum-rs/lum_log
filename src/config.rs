use std::collections::HashMap;

use lum_libs::{
    log::LevelFilter,
    serde::{Deserialize, Serialize},
};

/// Parts of the logger configuration that are meant to be user-configurable, and thus serializable and deserializable.
/// This is used by the [`setup`](crate::setup) function and the [`Builder`](crate::Builder) to set up the logger.
/// The idea is to implement `AsRef<Config>` for your own configuration type, and then use it to set up the logger.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Config {
    pub colors: HashMap<LevelFilter, String>,
    pub min_log_level: LevelFilter,
}

impl Default for Config {
    /// Returns a new instance of `Config` with the below default values.
    /// This is the same as [`defaults::config()`](crate::defaults::config).
    ///
    /// ### Colors
    /// | Level | Color  |
    /// |-------|--------|
    /// | Error | red    |
    /// | Warn  | yellow |
    /// | Info  | green  |
    /// | Debug | magenta|
    /// | Trace | cyan   |
    ///
    /// ### Minimum log level
    /// `Info`
    ///
    /// # Examples
    /// ```
    /// use lum_log::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.colors.len(), 5);
    /// assert_eq!(config.min_log_level, lum_libs::log::LevelFilter::Info);
    /// ```
    ///
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert(LevelFilter::Error, "red".to_string());
        colors.insert(LevelFilter::Warn, "yellow".to_string());
        colors.insert(LevelFilter::Info, "green".to_string());
        colors.insert(LevelFilter::Debug, "magenta".to_string());
        colors.insert(LevelFilter::Trace, "cyan".to_string());

        Config {
            colors,
            min_log_level: LevelFilter::Info,
        }
    }
}
