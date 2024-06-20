use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to update provider")]
    UpdateError,
}
