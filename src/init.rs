#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InitError {
    #[error("libatspi is already initialised")]
    AlreadyInitialised,
    #[error("Could not connect to the a11y bus")]
    NoA11yBus,
    #[error("Unknown error {0}")]
    Unknown(i32),
}

pub fn init() -> Result<(), InitError> {
    match unsafe { ffi::atspi_init() } {
        0 => Ok(()),
        1 => Err(InitError::AlreadyInitialised),
        2 => Err(InitError::NoA11yBus),
        x => Err(InitError::Unknown(x)),
    }
}
