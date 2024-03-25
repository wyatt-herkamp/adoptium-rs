use reqwest::Response;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdoptiumError {
    #[error("Reqwest had an Error {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde Json Parse Error {0}")]
    JSONError(#[from] serde_json::Error),
    #[error("Internal Error {0}")]
    Custom(String),
    #[error("A Bad Response Occurred")]
    BadResponse(Response),
    #[error("Invalid URL {0}")]
    InvalidUrl(#[from] url::ParseError),
}

impl From<ParseIntError> for AdoptiumError {
    fn from(err: ParseIntError) -> AdoptiumError {
        AdoptiumError::Custom(format!("{}", err))
    }
}

pub trait IntoResult {
    fn into_result(self) -> Result<Response, AdoptiumError>;
}

impl IntoResult for Response {
    fn into_result(self) -> Result<Response, AdoptiumError> {
        if self.status().is_success() {
            Ok(self)
        } else {
            Err(AdoptiumError::BadResponse(self))
        }
    }
}
