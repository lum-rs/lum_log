//! lum_log is a simple wrapper around log4rs offering easy setup and convenience macros.
//! It provides a simplified builder for log4rs configurations.
//! Furthermore, it provides logging macros that fall back to stdout/stderr if the logger is not set up yet.

/// Defines the [`ConfigBuilder`] for building log4rs configurations.
pub mod config;
/// Defines some defaults that help setting up logging.
pub mod default;
/// Defines functions to set up the logger.
pub mod logger;
/// Defines convenience logging macros.
pub mod macros;

/// Re-exports of external crates.
pub use lum_libs::log;

// Re-exports of internal modules.
pub use config::ConfigBuilder;
pub use logger::{is_set_up, setup};
