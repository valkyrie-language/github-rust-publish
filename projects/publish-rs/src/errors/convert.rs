#[cfg(feature = "wasm-opt")]
use wasm_opt::OptimizationError;
use crate::{GithubError, errors::LegionErrorKind};

impl From<std::io::Error> for GithubError {
    fn from(error: std::io::Error) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}

impl From<anyhow::Error> for GithubError {
    fn from(error: anyhow::Error) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}

impl From<wat::Error> for GithubError {
    fn from(error: wat::Error) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}

impl From<dialoguer::Error> for GithubError {
    fn from(error: dialoguer::Error) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}



#[cfg(feature = "wasm-opt")]
impl From<OptimizationError> for GithubError {
    fn from(error: OptimizationError) -> Self {
        let kind = LegionErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}
