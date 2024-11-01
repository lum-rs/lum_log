use lum_libs::log::SetLoggerError;

pub mod builder;
pub mod config;
pub mod defaults;
pub mod logger;
pub mod macros;

pub use builder::Builder;
pub use config::Config;
pub use logger::{is_set_up, setup};

/// Set up the logger using minimal configuration.
/// This will use the defaults defined in [`defaults`].
///
/// # Example
/// ```
/// use lum_log::{is_set_up, setup_with_defaults};
///
/// assert!(!is_set_up());
///
/// let result = setup_with_defaults();
///
/// assert!(result.is_ok());
/// assert!(is_set_up());
pub fn setup_with_defaults() -> Result<(), SetLoggerError> {
    Builder::new(defaults::format()).apply()?;

    Ok(())
}
