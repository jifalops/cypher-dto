use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Map did not contain {0}, and no default was provided")]
    MissingField(String),
    #[error("Map contained a value for {0}, but it was the wrong type")]
    TypeMismatch(String),
    /// Wraps one of the Map errors
    #[error("{0}")]
    FromMapInner(String),
    #[error("Error building field {1} on {0}")]
    BuilderError(String, String),
}
