use reqwest::{Response};
use std::num::ParseIntError;
use std::string::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdoptiumError {
    #[error("Reqwest had an Error {0}")]
    ReqwestError(reqwest::Error),
    #[error("Serde Json Parse Error {0}")]
    JSONError(serde_json::Error),
    #[error("Internal Error {0}")]
    Custom(String),
    #[error("A Bad Response Occurred")]
    BadResponse(Response),
}

impl From<reqwest::Error> for AdoptiumError {
    fn from(err: reqwest::Error) -> AdoptiumError {
        AdoptiumError::ReqwestError(err)
    }
}

impl From<ParseIntError> for AdoptiumError {
    fn from(err: ParseIntError) -> AdoptiumError {
        AdoptiumError::Custom(format!("{}", err))
    }
}

impl From<serde_json::Error> for AdoptiumError {
    fn from(err: serde_json::Error) -> AdoptiumError {
        AdoptiumError::JSONError(err)
    }
}

impl From<ParseError> for AdoptiumError {
    fn from(err: ParseError) -> AdoptiumError {
        AdoptiumError::Custom(format!("Unable to parse URL {}", err))
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
