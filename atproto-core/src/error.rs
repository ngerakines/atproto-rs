pub use anyhow::{Error, Result};

#[derive(Debug)]
pub struct AtProtoError {
    pub err: anyhow::Error,
}

impl From<anyhow::Error> for AtProtoError {
    fn from(err: anyhow::Error) -> AtProtoError {
        AtProtoError { err }
    }
}

impl From<serde_json::Error> for AtProtoError {
    fn from(err: serde_json::Error) -> AtProtoError {
        AtProtoError { err: err.into() }
    }
}

impl From<std::io::Error> for AtProtoError {
    fn from(err: std::io::Error) -> AtProtoError {
        AtProtoError { err: err.into() }
    }
}
