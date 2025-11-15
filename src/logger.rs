use lum_libs::{
    log::SetLoggerError,
    log4rs::{self, Config, Handle},
    parking_lot::Mutex,
};

static LOGGER_HANDLE: Mutex<Option<Handle>> = Mutex::new(None);

/// Returns whether the logger has been set up.
/// This uses an atomic boolean under the hood, so it is safe for concurrent use.
pub fn is_set_up() -> bool {
    LOGGER_HANDLE.lock().is_some()
}

/// Sets up the logger with the given [`Config`] and applies it as the global logger.
/// This uses [`log4rs`] under the hood.
/// You can call this multiple times to overwrite an existing logger's config.
pub fn setup(config: Config) -> Result<(), SetLoggerError> {
    let mut lock = LOGGER_HANDLE.lock();

    if let Some(handle) = lock.as_ref() {
        handle.set_config(config);
        return Ok(());
    }

    let handle = log4rs::init_config(config)?;
    *lock = Some(handle);
    Ok(())
}
